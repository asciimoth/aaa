use anyhow::Result;
use core::time;
use std::io::{self, StdoutLock, Write};
use std::sync::mpsc::channel;
use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
    time::Duration,
};

use rs3a::Art;

#[derive(Debug)]
pub struct FrameWithDelay {
    pub delay: time::Duration,
    pub rows: Vec<String>,
}

#[derive(Debug)]
pub struct Player {
    pub rx: Option<Receiver<()>>,
    pub frames: Vec<FrameWithDelay>,
    pub y_offset: usize,
    pub loop_flag: bool,
    pub height: usize,
}

impl Player {
    pub fn new(art: &Art, rx: Option<Receiver<()>>, y_offset: usize) -> Self {
        let mut frames = Vec::with_capacity(art.frames());
        let mut height = 0;
        for (i, frame) in art.to_ansi_frames().iter().enumerate() {
            let rows: Vec<String> = frame.lines().map(|s| String::from(s)).collect();
            height = height.max(rows.len());
            frames.push(FrameWithDelay {
                rows: rows,
                delay: time::Duration::from_millis(art.get_frame_delay(i) as u64),
            });
        }
        Self {
            rx,
            frames,
            y_offset,
            height,
            loop_flag: art.get_loop_key(),
        }
    }
    pub fn launch(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }
    pub fn run(&self) {
        'outer: loop {
            for frame in &self.frames {
                {
                    let stdout = io::stdout();
                    let mut handle = stdout.lock();
                    for row in &frame.rows {
                        write!(handle, "\x1b[{}C{}\x1b[0m\n", self.y_offset, row).unwrap();
                    }
                    self.frame_cleanup(&mut handle);
                    handle.flush().unwrap();
                }
                if self.wait(frame.delay) {
                    break 'outer;
                }
            }
            if !self.loop_flag {
                break;
            }
        }
    }
    fn frame_cleanup(&self, handle: &mut StdoutLock) {
        write!(handle, "\x1b[0m\r\x1b[{}A", self.height).unwrap();
    }
    fn wait(&self, delay: Duration) -> bool {
        if let Some(rx) = &self.rx {
            if let Ok(_) = rx.recv_timeout(delay) {
                true
            } else {
                false
            }
        } else {
            thread::sleep(delay);
            false
        }
    }
}

pub fn term_pre(title: &str, height: usize) {
    print!("\x1b]0;{}\x07\x1B[?25l", title);
    for _ in 0..height {
        println!("");
    }
    print!("\x1b[{}A", height);
}

pub fn term_post(height: usize) {
    for _ in 0..height {
        println!("");
    }
    print!("\x1B[?25h\r");
}

fn play_one(art: &Art, off: usize) -> Result<()> {
    let (tx1, rx1) = channel();
    ctrlc::set_handler(move || {
        let _ = tx1.clone().send(());
    })?;
    let player = Player::new(art, Some(rx1), off);
    term_pre(&art.title_line(), art.height());
    player.run();
    term_post(art.height());
    Ok(())
}

fn play_two(primary: &Art, secondary: &Art, off: usize, s_off: usize) -> Result<()> {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    let stop = tx2.clone();
    ctrlc::set_handler(move || {
        let _ = tx1.clone().send(());
        let _ = tx2.clone().send(());
    })?;

    let player1 = Player::new(primary, Some(rx1), off);
    let player2 = Player::new(secondary, Some(rx2), off + primary.width() + s_off);

    let h = primary.height().max(secondary.height());

    term_pre(&primary.title_line(), h);
    let handler = player2.launch();
    player1.run();
    let _ = stop.send(());
    handler.join().unwrap();
    term_post(h);

    Ok(())
}

pub fn play(primary: &Art, secondary: Option<&Art>, off: usize, s_off: usize) -> Result<()> {
    if let Some(secondary) = secondary {
        play_two(primary, secondary, off, s_off + 1)
    } else {
        play_one(primary, off)
    }
}
