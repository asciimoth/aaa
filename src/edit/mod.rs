mod set;

use anyhow::Result;
use argh::FromArgs;

use crate::{edit::set::CmdSet, loader::load};

/// Editing subcommands
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "edit")]
pub struct CmdEdit {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    #[argh(subcommand)]
    cmds: SubCmds,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCmds {
    Set(CmdSet),
}

impl CmdEdit {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        match &self.cmds {
            SubCmds::Set(cmd) => cmd.run(&mut art),
        }?;
        println!("{}", art.to_string());
        Ok(())
    }
}
