use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Print art back in 3a format
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-3a")]
pub struct CmdTo3a {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdTo3a {
    pub fn run(&self) -> Result<()> {
        println!("{}", load(&self.file)?.to_string());
        Ok(())
    }
}
