use std::{
    fs,
    io::{self, Read},
};

use anyhow::Result;
use argh::FromArgs;
use rs3a::Art;

/// Constructs art from plain text with ANSI color escape codes
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "from-text")]
pub struct CmdFromText {
    /// text file path (alternatively pipe text to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdFromText {
    pub fn run(&self) -> Result<()> {
        let txt = match &self.file {
            Some(path) => fs::read_to_string(path)?,
            None => {
                let mut txt = String::new();
                io::stdin().read_to_string(&mut txt)?;
                txt
            }
        };
        println!("{}", Art::from_ansi_text(&txt));
        Ok(())
    }
}
