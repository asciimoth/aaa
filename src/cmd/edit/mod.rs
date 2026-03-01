/*
    This file is part of aaa.

    aaa is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    aaa is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with aaa.  If not, see <https://www.gnu.org/licenses/>.
*/
pub mod color;
pub mod crop;
pub mod delay;
pub mod fill;
pub mod filter;
pub mod frame;
pub mod header;
pub mod pin;
pub mod print;
pub mod set;
pub mod strip;
pub mod tag;

use crate::{
    cmd::edit::{
        color::{ColorForceCmd, ColorMapCmd, ColorUnMapCmd, PaletteResetCmd},
        crop::CropCmd,
        delay::{DelayReSetCmd, DelaySetCmd},
        fill::{CleanCmd, FillAreaCmd, FillCmd},
        filter::FilterCmd,
        frame::{
            FrameDedupCmd, FrameDupCmd, FrameRemoveCmd, FrameRevCmd, FrameRotBackCmd,
            FrameRotForthCmd, FrameSliceCmd, FrameSureCmd, FrameSwapCmd,
        },
        header::{
            AuthorsCmd, EditorCmd, LicenseCmd, LoopCmd, OrigsCmd, PreviewCmd, SrcCmd, TitleCmd,
        },
        pin::{PinColorCmd, PinTextCmd},
        print::{PrintANSICmd, PrintCmd},
        set::SetCmd,
        tag::{TagAddCmd, TagRmCmd, TagsDropCmd},
    },
    loader::load,
};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct EditCmd {
    /// art file path (alternatively pipe art to stdin)
    file: Option<String>,

    #[command(subcommand)]
    command: EditSubcommands,
}

#[derive(Debug, Subcommand)]
enum EditSubcommands {
    /// Set cell (text, color)
    Set(SetCmd),
    /// Strip comments from art
    Strip(strip::StripCmd),
    /// Search or add new color mapping.
    /// Mapped color prints to stderr.
    ColorMap(ColorMapCmd),
    /// Remove color mapping.
    ColorUnmap(ColorUnMapCmd),
    /// Reset palette.
    PaletteReset(PaletteResetCmd),
    /// Force enable/disable colors
    ColorForce(ColorForceCmd),
    /// Crop art
    Crop(CropCmd),
    /// Set delay for whole art or for specific frame
    DelaySet(DelaySetCmd),
    /// Reset all art delays to default (50 milis)
    DelayReset(DelayReSetCmd),
    /// Fill all frames or specific one with text and color
    Fill(FillCmd),
    /// Fill area in all frames or specific one with text and color
    FillArea(FillAreaCmd),
    /// Fill all frames or specific one with default text, color
    Clean(CleanCmd),
    /// Pin text channel
    PinText(PinTextCmd),
    /// Pin color channel
    PinColor(PinColorCmd),
    /// Print text to art
    Print(PrintCmd),
    /// Print text with ansi color codes to art
    PrintAnsi(PrintANSICmd),
    /// Filter art with arbitrary program
    #[command(
        after_help = "FILTERING:\n  Provided command will be executed for each art frame and\n  command's output will be parsed back.\nINPUT TYPES:\n  Depending on input type different forms of frame data will be passed to cmd:\n  - text - plaintext without colors\n  - ansi - plaintext with ANSI color codes\n  - frame - raw 3a frame data\nEXAMPLE:\n  $ aaa edit apple filter text -- lolcat -f | aaa play"
    )]
    Filter(FilterCmd),
    /// Remove frame
    FrameRemove(FrameRemoveCmd),
    /// Duplicate frame
    FrameDup(FrameDupCmd),
    /// Ensures a frame exists at the given index,
    /// creating new frames if necessary
    FrameEnsure(FrameSureCmd),
    /// Remove all frames out of inclusive subrange
    FramesSlice(FrameSliceCmd),
    /// Swap two frames
    FramesSwap(FrameSwapCmd),
    /// Reverse frames
    FramesReverse(FrameRevCmd),
    /// Deduplicate frames
    FrameDedup(FrameDedupCmd),
    /// Rotate frames forth
    FrameRF(FrameRotForthCmd),
    /// Rotate frames back
    FrameRB(FrameRotBackCmd),

    /// Set art title
    Title(TitleCmd),
    /// Set authors
    Authors(AuthorsCmd),
    /// Set orig authors
    Origs(OrigsCmd),
    /// Set src
    Src(SrcCmd),
    /// Set editor
    Editor(EditorCmd),
    /// Set license
    License(LicenseCmd),
    /// Set loop
    Loop(LoopCmd),
    /// Set preview frame
    Preview(PreviewCmd),

    /// Add tag to art
    TagAdd(TagAddCmd),
    /// Remove tag from art
    TagRm(TagRmCmd),
    /// Drop all tags
    TagsDrop(TagsDropCmd),
}

impl EditCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut art = load(&self.file)?;
        match &self.command {
            EditSubcommands::Set(cmd) => cmd.run(&mut art),
            EditSubcommands::Strip(cmd) => cmd.run(&mut art),
            EditSubcommands::ColorMap(cmd) => cmd.run(&mut art),
            EditSubcommands::ColorUnmap(cmd) => cmd.run(&mut art),
            EditSubcommands::PaletteReset(cmd) => cmd.run(&mut art),
            EditSubcommands::ColorForce(cmd) => cmd.run(&mut art),
            EditSubcommands::Crop(cmd) => cmd.run(&mut art),
            EditSubcommands::DelaySet(cmd) => cmd.run(&mut art),
            EditSubcommands::DelayReset(cmd) => cmd.run(&mut art),
            EditSubcommands::Fill(cmd) => cmd.run(&mut art),
            EditSubcommands::FillArea(cmd) => cmd.run(&mut art),
            EditSubcommands::Clean(cmd) => cmd.run(&mut art),
            EditSubcommands::PinText(cmd) => cmd.run(&mut art),
            EditSubcommands::PinColor(cmd) => cmd.run(&mut art),
            EditSubcommands::Print(cmd) => cmd.run(&mut art),
            EditSubcommands::PrintAnsi(cmd) => cmd.run(&mut art),
            EditSubcommands::Filter(cmd) => cmd.run(&mut art),
            EditSubcommands::FrameRemove(cmd) => cmd.run(&mut art),
            EditSubcommands::FrameDup(cmd) => cmd.run(&mut art),
            EditSubcommands::FrameEnsure(cmd) => cmd.run(&mut art),
            EditSubcommands::FramesSlice(cmd) => cmd.run(&mut art),
            EditSubcommands::FramesSwap(cmd) => cmd.run(&mut art),
            EditSubcommands::FramesReverse(cmd) => cmd.run(&mut art),
            EditSubcommands::FrameDedup(cmd) => cmd.run(&mut art),
            EditSubcommands::FrameRF(cmd) => cmd.run(&mut art),
            EditSubcommands::FrameRB(cmd) => cmd.run(&mut art),
            EditSubcommands::TagAdd(cmd) => cmd.run(&mut art),
            EditSubcommands::TagRm(cmd) => cmd.run(&mut art),
            EditSubcommands::TagsDrop(cmd) => cmd.run(&mut art),
            EditSubcommands::Title(cmd) => cmd.run(&mut art),
            EditSubcommands::Authors(cmd) => cmd.run(&mut art),
            EditSubcommands::Origs(cmd) => cmd.run(&mut art),
            EditSubcommands::Src(cmd) => cmd.run(&mut art),
            EditSubcommands::Editor(cmd) => cmd.run(&mut art),
            EditSubcommands::License(cmd) => cmd.run(&mut art),
            EditSubcommands::Loop(cmd) => cmd.run(&mut art),
            EditSubcommands::Preview(cmd) => cmd.run(&mut art),
        }?;
        println!("{}", art.to_string());
        Ok(())
    }
}
