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
mod edit;
mod from_text;
mod generate;
mod img;
mod list;
mod loader;
mod play;
mod player;
mod preview;
mod strip;
mod to_3a;
mod to_cast;
mod to_dur;
mod to_frames;
mod to_gif;
mod to_json;
mod to_mp4;
mod to_png;
mod to_svg;
mod to_ttyrec;
mod to_webp;

use crate::from_text::CmdFromText;
use crate::to_dur::CmdToDur;
use crate::to_gif::CmdToGif;
use crate::to_mp4::CmdToMp4;
use crate::to_png::CmdToPng;
use crate::to_ttyrec::CmdToTtyrec;
use crate::to_webp::CmdToWebp;
use crate::{
    edit::CmdEdit, generate::CmdGen, list::CmdList, play::CmdPlay, preview::CmdPreview,
    strip::CmdStrip, to_3a::CmdTo3a, to_cast::CmdToCast, to_frames::CmdToFrames,
    to_json::CmdToJson, to_svg::CmdToSvg,
};
use anyhow::Result;
use argh::FromArgs;

/// Animated ASCII art tool by asciimoth
#[derive(FromArgs, PartialEq, Debug)]
pub struct Cmd {
    #[argh(subcommand)]
    cmds: SubCmds,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCmds {
    Play(CmdPlay),
    Preview(CmdPreview),
    List(CmdList),
    Strip(CmdStrip),
    Edit(CmdEdit),
    Gen(CmdGen),
    FromTxt(CmdFromText),
    ToFrames(CmdToFrames),
    ToCast(CmdToCast),
    To3a(CmdTo3a),
    ToSvg(CmdToSvg),
    ToJson(CmdToJson),
    ToDur(CmdToDur),
    ToTtyrec(CmdToTtyrec),
    ToPng(CmdToPng),
    ToGif(CmdToGif),
    ToWebp(CmdToWebp),
    ToMp4(CmdToMp4),
}

impl SubCmds {
    pub fn run(&self) -> Result<()> {
        match self {
            SubCmds::Play(cmd) => cmd.run(),
            SubCmds::Preview(cmd) => cmd.run(),
            SubCmds::ToFrames(cmd) => cmd.run(),
            SubCmds::ToCast(cmd) => cmd.run(),
            SubCmds::To3a(cmd) => cmd.run(),
            SubCmds::ToSvg(cmd) => cmd.run(),
            SubCmds::ToJson(cmd) => cmd.run(),
            SubCmds::ToDur(cmd) => cmd.run(),
            SubCmds::ToTtyrec(cmd) => cmd.run(),
            SubCmds::ToPng(cmd) => cmd.run(),
            SubCmds::ToGif(cmd) => cmd.run(),
            SubCmds::ToWebp(cmd) => cmd.run(),
            SubCmds::ToMp4(cmd) => cmd.run(),
            SubCmds::List(cmd) => cmd.run(),
            SubCmds::Strip(cmd) => cmd.run(),
            SubCmds::Edit(cmd) => cmd.run(),
            SubCmds::Gen(cmd) => cmd.run(),
            SubCmds::FromTxt(cmd) => cmd.run(),
        }
    }
}

fn main() -> Result<()> {
    let cmd: Cmd = argh::from_env();
    cmd.cmds.run()?;
    Ok(())
}
