mod color;
mod crop;
mod fill;
mod frame;
mod pin;
mod set;

use anyhow::Result;
use argh::FromArgs;

use crate::{
    edit::{
        color::{CmdColorForce, CmdColorMap, CmdColorUnMap, CmdPaletteRest},
        crop::CmdCrop,
        fill::{CmdClean, CmdFill, CmdFillArea},
        frame::{
            CmdFrameDedup, CmdFrameDup, CmdFrameRemove, CmdFrameRev, CmdFrameRotBack,
            CmdFrameRotForth, CmdFrameSlice, CmdFrameSure, CmdFrameSwap,
        },
        pin::{CmdPinColor, CmdPinText},
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
    FrameRemove(CmdFrameRemove),
    FrameDup(CmdFrameDup),
    FrameSure(CmdFrameSure),
    FrameSlice(CmdFrameSlice),
    FrameSwap(CmdFrameSwap),
    FrameRev(CmdFrameRev),
    FrameDedup(CmdFrameDedup),
    FrameRotF(CmdFrameRotForth),
    FrameRotB(CmdFrameRotBack),
    PinText(CmdPinText),
    PinColor(CmdPinColor),
    Crop(CmdCrop),
    Fill(CmdFill),
    FillArea(CmdFillArea),
    Clean(CmdClean),
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
            SubCmds::FrameRemove(cmd) => cmd.run(&mut art),
            SubCmds::FrameDup(cmd) => cmd.run(&mut art),
            SubCmds::FrameSure(cmd) => cmd.run(&mut art),
            SubCmds::FrameSlice(cmd) => cmd.run(&mut art),
            SubCmds::FrameSwap(cmd) => cmd.run(&mut art),
            SubCmds::FrameRev(cmd) => cmd.run(&mut art),
            SubCmds::FrameDedup(cmd) => cmd.run(&mut art),
            SubCmds::FrameRotF(cmd) => cmd.run(&mut art),
            SubCmds::FrameRotB(cmd) => cmd.run(&mut art),
            SubCmds::PinText(cmd) => cmd.run(&mut art),
            SubCmds::PinColor(cmd) => cmd.run(&mut art),
            SubCmds::Crop(cmd) => cmd.run(&mut art),
            SubCmds::Fill(cmd) => cmd.run(&mut art),
            SubCmds::FillArea(cmd) => cmd.run(&mut art),
            SubCmds::Clean(cmd) => cmd.run(&mut art),
        }?;
        println!("{}", art.to_string());
        Ok(())
    }
}
