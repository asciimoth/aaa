use std::{
    io::{self, Write},
    sync::mpsc::channel,
    thread,
};

use anyhow::Result;
use argh::FromArgs;

use crate::{frames::art2frames, loader::load};

/// Play art in terminal
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "play")]
pub struct CmdPlay {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// whether loop aniamtion
    #[argh(option, long = "loop")]
    loop_flag: Option<bool>,
}

impl CmdPlay {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }
        let frames = art2frames(&art);
        print!("\x1B[?25l"); // Disable cursor
        io::stdout().flush().unwrap();
        let loop_flag = match self.loop_flag {
            Some(flag) => flag,
            None => art.get_loop_key(),
        };
        let (tx, rx) = channel();
        ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))?;
        let mut last_frame = 0;
        'outer: loop {
            for (i, frame) in frames.iter().enumerate() {
                println!("{}", frame.frame);
                io::stdout().flush().unwrap();
                last_frame = i;
                thread::sleep(frame.delay);
                if let Ok(_) = rx.recv_timeout(frame.delay) {
                    break 'outer;
                }
            }
            if !loop_flag {
                break;
            }
        }
        if frames.len() > 0 {
            println!("\r{}", frames[last_frame].frame);
        }
        io::stdout().flush().unwrap();
        for _ in 0..art.height() + 1 {
            println!("");
        }
        print!("\x1B[?25h"); // Enable cursor
        io::stdout().flush().unwrap();
        Ok(())
    }
}
