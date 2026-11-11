/*
    This file is part of aaa.

    aaa is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    aaa is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with aaa.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::{
    collections::HashMap,
    fmt,
    io::Write,
    process::{Command, Stdio},
};

use crate::parallel::FrameJobs;
use anyhow::{Result, anyhow, ensure};
use rs3a::Frame;

#[derive(clap::ValueEnum, PartialEq, Debug, Copy, Clone)]
enum FilterInput {
    Text,
    Ansi,
    Frame,
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FilterCmd {
    /// filter input type
    #[arg(value_enum)]
    input: FilterInput,

    #[command(flatten)]
    jobs: FrameJobs,

    /// Command to run per frame (use --jobs 1 if it depends on execution order)
    cmd: Vec<String>,
}

fn env(art: &rs3a::Art) -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert(String::from("AAA_FRAMES"), format!("{}", art.frames()));
    env.insert(String::from("AAA_WIDTH"), format!("{}", art.width()));
    env.insert(String::from("AAA_HEIGHT"), format!("{}", art.height()));
    env.insert(String::from("AAA_COLOR"), format!("{}", art.color()));
    env
}

impl FilterCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        ensure!(!self.cmd.is_empty(), "Empty command");
        let mut base_env = env(art);
        base_env.insert(
            String::from("AAA_INPUT"),
            match self.input {
                FilterInput::Text => "text",
                FilterInput::Ansi => "ansi",
                FilterInput::Frame => "frame",
            }
            .to_owned(),
        );
        // Read all inputs before applying results, so workers see the original art.
        let inputs = crate::progress::during("prepare filter input", || match self.input {
            FilterInput::Ansi => art.to_ansi_frames(),
            FilterInput::Text | FilterInput::Frame => (0..art.frames())
                .map(|f| {
                    let frame = art.frame(f).unwrap();
                    if self.input == FilterInput::Text {
                        format!("{}", ColorlessFramePrinter { frame })
                    } else {
                        format!("{}", frame)
                    }
                })
                .collect(),
        });
        let delays: Vec<_> = (0..art.frames()).map(|f| art.get_frame_delay(f)).collect();
        let height = art.height();
        self.jobs.run(
            art.frames(),
            "filtering",
            |f, task| {
                let mut env = base_env.clone();
                env.insert(String::from("AAA_FRAME"), f.to_string());
                env.insert(String::from("AAA_DELAY"), delays[f].to_string());
                let filtered = run_cmd_with_input(&self.cmd, &inputs[f], &env, task)?;
                ensure!(
                    filtered.lines().count() >= height,
                    "filter returned fewer than {height} lines"
                );
                Ok(filtered)
            },
            |f, filtered| {
                // Palette allocation and frame updates must stay in frame order.
                for (r, line) in filtered.lines().take(height).enumerate() {
                    art.print_ansi(f, 0, r, line);
                }
                Ok(())
            },
        )
    }
}

fn run_cmd_with_input(
    cmd: &[String],
    input: &str,
    env_vars: &HashMap<String, String>,
    task: &zprs::Node,
) -> Result<String> {
    // Split the command into program and arguments
    let mut iter = cmd.iter();
    let program = iter.next().ok_or(anyhow!("Empty command"))?;
    let args: Vec<_> = iter.collect();

    // Configure the command: pipe all three stdio streams
    let mut command = Command::new(program);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    // Set environment variables (inherits the parent’s environment by default)
    command.envs(env_vars);

    // Spawn the command
    let process = task.start(program, 0);
    let mut child = process.spawn(command)?;

    // Obtain a handle to the child’s stdin
    let mut stdin = child.stdin.take().expect("failed to get stdin handle");

    // Write the input string to the child’s stdin in a separate thread.
    // This prevents a potential deadlock if the child blocks while reading.
    let input_owned = input.to_owned();
    let write_handle = std::thread::Builder::new()
        .name("filter stdin".into())
        .spawn(move || -> std::io::Result<()> {
            stdin.write_all(input_owned.as_bytes())?;
            stdin.flush()?; // ensure all data is written
            // Dropping `stdin` here closes the pipe, signaling EOF to the child.
            Ok(())
        });
    let write_handle = match write_handle {
        Ok(handle) => handle,
        Err(error) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error.into());
        }
    };

    // Wait for the command to finish and capture its output
    let output = child.wait_with_output();

    // Wait for the stdin thread to finish and propagate any I/O errors
    let written = write_handle
        .join()
        .map_err(|_| anyhow!("filter stdin thread panicked"))?;
    let output = output?;

    // Check the exit status
    if output.status.success() {
        written?;
        // Convert stdout to a String (assumes UTF‑8 encoding)
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    } else {
        // Build a meaningful error message using the exit code and stderr
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        Err(anyhow!(format!(
            "Command failed with exit code {}: {}",
            exit_code, stderr
        )))
    }
}

struct ColorlessFramePrinter {
    frame: Frame,
}

impl fmt::Display for ColorlessFramePrinter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.frame.fmt_with_colors(f, Some(false))
    }
}
