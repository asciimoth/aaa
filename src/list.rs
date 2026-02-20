use anyhow::Result;
use argh::FromArgs;

use crate::loader::BuiltIn;

/// List all builtin art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "list")]
pub struct CmdList {}

impl CmdList {
    pub fn run(&self) -> Result<()> {
        for art in BuiltIn::iter() {
            println!("{}", art);
        }
        Ok(())
    }
}
