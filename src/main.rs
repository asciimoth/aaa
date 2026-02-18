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
mod play_cmd;

use crate::play_cmd::CmdPlay;
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
}

impl SubCmds {
    pub fn run(&self) -> Result<()> {
        match self {
            SubCmds::Play(cmd) => cmd.run(),
        }
    }
}

fn main() -> Result<()> {
    let cmd: Cmd = argh::from_env();
    cmd.cmds.run()?;
    Ok(())
}
