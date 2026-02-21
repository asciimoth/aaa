use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Convert art to json document
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-json")]
pub struct CmdToJson {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdToJson {
    pub fn run(&self) -> Result<()> {
        println!("{}", load(&self.file)?.to_json());
        Ok(())
    }
}
