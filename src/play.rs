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
use anyhow::Result;
use argh::FromArgs;

use crate::{loader::load, player::play};

/// Play art (or two side by side) in terminal
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "play")]
pub struct CmdPlay {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// secondary art file path
    #[argh(option)]
    secondary: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// whether loop aniamtion
    #[argh(option, long = "loop")]
    loop_flag: Option<bool>,

    /// horizontal offset
    #[argh(option, short = 'o', default = "0")]
    offset: usize,

    /// secondary art horizontal offset
    #[argh(option, default = "0")]
    secondary_offset: usize,
}

impl CmdPlay {
    pub fn run(&self) -> Result<()> {
        let primary = {
            let mut art = load(&self.file)?;
            if self.no_colors {
                art.set_colors_key(Some(false));
            }
            if let Some(flag) = self.loop_flag {
                art.set_loop_key(flag);
            }
            art
        };
        let secondary = match &self.secondary {
            Some(file) => {
                let mut art = load(&Some(file.clone()))?;
                if self.no_colors {
                    art.set_colors_key(Some(false));
                }
                if let Some(flag) = self.loop_flag {
                    art.set_loop_key(flag);
                }
                Some(art)
            }
            None => None,
        };
        if primary.frames() < 1 {
            return Ok(());
        }

        play(
            &primary,
            secondary.as_ref(),
            self.offset,
            self.secondary_offset,
        )?;
        Ok(())
    }
}
