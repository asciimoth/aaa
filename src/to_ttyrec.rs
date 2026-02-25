use std::io::{Write, stdout};

use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Convert art to ttyrec format
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-ttyrec")]
pub struct CmdToTtyrec {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdToTtyrec {
    pub fn run(&self) -> Result<()> {
        let mut out = stdout();
        out.write(&load(&self.file)?.to_ttyrec())?;
        Ok(())
    }
}
