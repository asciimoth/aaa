mod color;
mod crop;
mod delay;
pub mod effects;
mod fill;
mod filter;
mod frame;
mod header;
mod pin;
mod print;
mod set;
mod tag;

use anyhow::Result;
use argh::FromArgs;

use crate::{
    edit::{
        color::{CmdColorForce, CmdColorMap, CmdColorUnMap, CmdPaletteRest},
        crop::CmdCrop,
        delay::{CmdDelayReSet, CmdDelaySet},
        fill::{CmdClean, CmdFill, CmdFillArea},
        filter::CmdFilter,
        frame::{
            CmdFrameDedup, CmdFrameDup, CmdFrameRemove, CmdFrameRev, CmdFrameRotBack,
            CmdFrameRotForth, CmdFrameSlice, CmdFrameSure, CmdFrameSwap,
        },
        header::{
            CmdAuthors, CmdEditor, CmdLicense, CmdLoop, CmdOrigs, CmdPreview, CmdSrc, CmdTitle,
        },
        pin::{CmdPinColor, CmdPinText},
        print::CmdPrint,
        set::CmdSet,
        tag::{CmdTagAdd, CmdTagRm, CmdTagsDrop},
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
    Print(CmdPrint),
    Filter(CmdFilter),
    TagAdd(CmdTagAdd),
    TagRm(CmdTagRm),
    TagsDrop(CmdTagsDrop),
    DelaySet(CmdDelaySet),
    DelayReSet(CmdDelayReSet),
    Title(CmdTitle),
    Authors(CmdAuthors),
    Origs(CmdOrigs),
    Src(CmdSrc),
    Editor(CmdEditor),
    License(CmdLicense),
    Loop(CmdLoop),
    Preview(CmdPreview),
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
            SubCmds::Print(cmd) => cmd.run(&mut art),
            SubCmds::Filter(cmd) => cmd.run(&mut art),
            SubCmds::TagAdd(cmd) => cmd.run(&mut art),
            SubCmds::TagRm(cmd) => cmd.run(&mut art),
            SubCmds::TagsDrop(cmd) => cmd.run(&mut art),
            SubCmds::DelaySet(cmd) => cmd.run(&mut art),
            SubCmds::DelayReSet(cmd) => cmd.run(&mut art),
            SubCmds::Title(cmd) => cmd.run(&mut art),
            SubCmds::Authors(cmd) => cmd.run(&mut art),
            SubCmds::Origs(cmd) => cmd.run(&mut art),
            SubCmds::Src(cmd) => cmd.run(&mut art),
            SubCmds::Editor(cmd) => cmd.run(&mut art),
            SubCmds::License(cmd) => cmd.run(&mut art),
            SubCmds::Loop(cmd) => cmd.run(&mut art),
            SubCmds::Preview(cmd) => cmd.run(&mut art),
        }?;
        println!("{}", art.to_string());
        Ok(())
    }
}
