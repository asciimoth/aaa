mod color;
mod set;

use anyhow::Result;
use argh::FromArgs;

use crate::{
    edit::{
        color::{CmdColorForce, CmdColorMap, CmdColorUnMap, CmdPaletteRest},
        set::CmdSet,
    },
    loader::load,
};

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
    ColorMap(CmdColorMap),
    ColorUnMap(CmdColorUnMap),
    ColorForce(CmdColorForce),
    Palette(CmdPaletteRest),
}

impl CmdEdit {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        match &self.cmds {
            SubCmds::Set(cmd) => cmd.run(&mut art),
            SubCmds::ColorMap(cmd) => cmd.run(&mut art),
            SubCmds::ColorUnMap(cmd) => cmd.run(&mut art),
            SubCmds::ColorForce(cmd) => cmd.run(&mut art),
            SubCmds::Palette(cmd) => cmd.run(&mut art),
        }?;
        println!("{}", art.to_string());
        Ok(())
    }
}
