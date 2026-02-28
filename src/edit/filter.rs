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

use anyhow::{Result, anyhow};
use argh::{FromArgValue, FromArgs};
use rs3a::Frame;

#[derive(FromArgValue, PartialEq, Debug)]
enum FilterInput {
    Text,
    Ansi,
    Frame,
}

/// Filter art with arbitrary program
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "filter")]
pub struct CmdFilter {
    /// filter input type: text | ansi | frame
    #[argh(positional)]
    input: FilterInput,

    /// command
    #[argh(positional)]
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

impl CmdFilter {
    fn run_text(&self, art: &mut rs3a::Art) -> Result<()> {
        let mut env = env(art);
        env.insert(String::from("AAA_INPUT"), String::from("text"));
        for f in 0..art.frames() {
            env.insert(String::from("AAA_FRAME"), format!("{}", f));
            env.insert(
                String::from("AAA_DELAY"),
                format!("{}", art.get_frame_delay(f)),
            );
            eprintln!("  {}/{}", f, art.frames());
            let frame = art.frame(f).unwrap();
            let input = format!("{}", ColorlessFramePrinter { frame });
            let filtered = run_cmd_with_input(&self.cmd, &input, &env)?;
            let lines: Vec<_> = filtered.lines().collect();
            for r in 0..art.height() {
                art.print_ansi(f, 0, r, lines[r]);
            }
        }
        Ok(())
    }
    fn run_frame(&self, art: &mut rs3a::Art) -> Result<()> {
        let mut env = env(art);
        env.insert(String::from("AAA_INPUT"), String::from("frame"));
        for f in 0..art.frames() {
            env.insert(String::from("AAA_FRAME"), format!("{}", f));
            env.insert(
                String::from("AAA_DELAY"),
                format!("{}", art.get_frame_delay(f)),
            );
            eprintln!("  {}/{}", f, art.frames());
            let frame = art.frame(f).unwrap();
            let input = format!("{}", frame);
            let filtered = run_cmd_with_input(&self.cmd, &input, &env)?;
            let lines: Vec<_> = filtered.lines().collect();
            for r in 0..art.height() {
                art.print_ansi(f, 0, r, lines[r]);
            }
        }
        Ok(())
    }
    fn run_ansi(&self, art: &mut rs3a::Art) -> Result<()> {
        let mut env = env(art);
        env.insert(String::from("AAA_INPUT"), String::from("ansi"));
        for (f, input) in art.to_ansi_frames().iter().enumerate() {
            env.insert(String::from("AAA_FRAME"), format!("{}", f));
            env.insert(
                String::from("AAA_DELAY"),
                format!("{}", art.get_frame_delay(f)),
            );
            eprintln!("  {}/{}", f, art.frames());
            let filtered = run_cmd_with_input(&self.cmd, &input, &env)?;
            let lines: Vec<_> = filtered.lines().collect();
            for r in 0..art.height() {
                art.print_ansi(f, 0, r, lines[r]);
            }
        }
        Ok(())
    }
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        eprintln!("filtering:");
        match self.input {
            FilterInput::Text => self.run_text(art),
            FilterInput::Ansi => self.run_ansi(art),
            FilterInput::Frame => self.run_frame(art),
        }
    }
}

fn run_cmd_with_input(
    cmd: &[String],
    input: &str,
    env_vars: &HashMap<String, String>,
) -> Result<String> {
    // Split the command into program and arguments
    let mut iter = cmd.into_iter();
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
    let mut child = command.spawn()?;

    // Obtain a handle to the child’s stdin
    let mut stdin = child.stdin.take().expect("failed to get stdin handle");

    // Write the input string to the child’s stdin in a separate thread.
    // This prevents a potential deadlock if the child blocks while reading.
    let input_owned = input.to_owned();
    let write_handle = std::thread::spawn(move || -> std::io::Result<()> {
        stdin.write_all(input_owned.as_bytes())?;
        stdin.flush()?; // ensure all data is written
        // Dropping `stdin` here closes the pipe, signaling EOF to the child.
        Ok(())
    });

    // Wait for the command to finish and capture its output
    let output = child.wait_with_output()?;

    // Wait for the stdin thread to finish and propagate any I/O errors
    write_handle.join().expect("stdin thread panicked")?;

    // Check the exit status
    if output.status.success() {
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
