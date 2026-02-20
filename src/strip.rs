use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Strip comments from art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "strip")]
pub struct CmdStrip {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdStrip {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        art.strip_comments();
        println!("{}", art.to_string());
        Ok(())
    }
}
