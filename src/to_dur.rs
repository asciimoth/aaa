use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Convert art to durformat (durdraw's ascii art format)
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-dur")]
pub struct CmdToDur {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdToDur {
    pub fn run(&self) -> Result<()> {
        println!("{}", load(&self.file)?.to_dur());
        Ok(())
    }
}
