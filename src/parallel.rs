use std::{io::Write, sync::Mutex, thread};

use anyhow::{Context, Result};
use rayon::prelude::*;
use zprs::Node;

#[derive(clap::Args, Debug, PartialEq)]
pub struct FrameJobs {
    /// Number of frames to process at once (0: available CPU threads, 1: sequential)
    #[arg(short = 'j', long, default_value_t = 0)]
    jobs: usize,
}

impl FrameJobs {
    /// Process bounded batches in parallel, then consume each batch in frame order.
    /// Only the consumer may change shared state such as the art palette or encoder.
    pub fn run<T: Send>(
        &self,
        frames: usize,
        label: &str,
        process: impl Fn(usize, &Node) -> Result<T> + Sync,
        mut consume: impl FnMut(usize, T) -> Result<()>,
    ) -> Result<()> {
        let workers = if self.jobs == 0 {
            thread::available_parallelism().map_or(1, usize::from)
        } else {
            self.jobs
        }
        .min(frames.max(1));
        let pool = if workers > 1 {
            Some(
                rayon::ThreadPoolBuilder::new()
                    .num_threads(workers)
                    .build()?,
            )
        } else {
            None
        };
        let task = crate::progress::start(label, frames);
        let fallback = !task.enabled();
        if fallback {
            eprintln!("{label}:");
        }
        let progress = Progress {
            completed: Mutex::new(0),
            frames,
            fallback,
        };
        let process = |frame| {
            let worker = task.start(&format!("frame {frame}"), 0);
            let tracked = worker.enabled();
            let value = process(frame, &worker).with_context(|| format!("frame {frame}"))?;
            // Ending this node completes one item in the parent. Do not also
            // increment the parent: zprs does this when the last handle drops.
            drop(worker);
            // The protocol has a fixed node limit. Count workers which could
            // not get a visible node as well.
            if !tracked {
                task.complete_one();
            }
            progress.advance()?;
            Ok(value)
        };
        for start in (0..frames).step_by(workers) {
            let end = start.saturating_add(workers).min(frames);
            let values: Vec<T> = if let Some(pool) = &pool {
                pool.install(|| {
                    (start..end)
                        .into_par_iter()
                        .map(&process)
                        .collect::<Result<_>>()
                })?
            } else {
                vec![process(start)?]
            };
            for (frame, value) in (start..end).zip(values) {
                consume(frame, value).with_context(|| format!("frame {frame}"))?;
            }
        }
        Ok(())
    }
}

struct Progress {
    completed: Mutex<usize>,
    frames: usize,
    fallback: bool,
}

impl Progress {
    fn advance(&self) -> std::io::Result<()> {
        if !self.fallback {
            return Ok(());
        }
        let mut completed = self.completed.lock().unwrap();
        *completed += 1;
        let mut stderr = std::io::stderr().lock();
        write!(stderr, "\r  {}/{}", *completed, self.frames)?;
        stderr.flush()
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        if self.fallback {
            let _ = writeln!(std::io::stderr());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        sync::{
            atomic::{AtomicUsize, Ordering},
            mpsc,
        },
        time::Duration,
    };

    #[test]
    fn processes_concurrently_but_consumes_in_order_with_bounded_batches() {
        let (tx, rx) = mpsc::channel();
        let rx = Mutex::new(rx);
        let completed = Mutex::new(Vec::new());
        let consumed = AtomicUsize::new(0);
        let mut output = Vec::new();
        FrameJobs { jobs: 2 }
            .run(
                6,
                "test",
                |frame, _| {
                    assert!(frame < consumed.load(Ordering::SeqCst) + 2);
                    if frame % 2 == 0 {
                        rx.lock().unwrap().recv_timeout(Duration::from_secs(5))?;
                        completed.lock().unwrap().push(frame);
                    } else {
                        completed.lock().unwrap().push(frame);
                        tx.send(())?;
                    }
                    Ok(frame)
                },
                |frame, value| {
                    assert_eq!(frame, value);
                    output.push(value);
                    consumed.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                },
            )
            .unwrap();
        assert_eq!(output, [0, 1, 2, 3, 4, 5]);
        assert_eq!(*completed.lock().unwrap(), [1, 0, 3, 2, 5, 4]);
    }

    #[test]
    fn sequential_processing_stops_at_first_error() {
        let mut output = Vec::new();
        let error = FrameJobs { jobs: 1 }
            .run(
                4,
                "test",
                |frame, _| {
                    anyhow::ensure!(frame != 2, "filter failed");
                    Ok(frame)
                },
                |_, value| {
                    output.push(value);
                    Ok(())
                },
            )
            .unwrap_err();
        assert_eq!(output, [0, 1]);
        assert_eq!(format!("{error:#}"), "frame 2: filter failed");
    }
}
