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
mod frames;
mod loader;
mod play;
mod preview;
mod to_3a;
mod to_cast;
mod to_frames;
mod to_json;
mod to_svg;

use crate::{
    play::CmdPlay, preview::CmdPreview, to_3a::CmdTo3a, to_cast::CmdToCast, to_frames::CmdToFrames,
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
    ToFrames(CmdToFrames),
    ToCast(CmdToCast),
    To3a(CmdTo3a),
    ToSvg(CmdToSvg),
    ToJson(CmdToJson),
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
        }
    }
}

fn main() -> Result<()> {
    let cmd: Cmd = argh::from_env();
    cmd.cmds.run()?;
    Ok(())
}
