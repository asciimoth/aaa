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
use crate::{
    loader::{load, load_with_fallback},
    player::play,
};
use anyhow::Result;
use clap::Args;

#[derive(Args, PartialEq, Debug)]
pub struct PlayCmd {
    /// art file path (alternatively pipe art to stdin)
    file: Option<String>,

    /// alternative art path in case primary don't exist of broken.
    #[arg(long, value_name = "FILE")]
    fallback_file: Option<String>,

    /// secondary art file path
    #[arg(long, value_name = "FILE")]
    secondary: Option<String>,

    /// disable colors
    #[arg(short = 'n', long)]
    no_colors: bool,

    /// whether loop animation
    #[clap(short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// horizontal offset
    #[arg(short = 'o', long, default_value_t = 0)]
    offset: usize,

    /// secondary art horizontal offset
    #[arg(long, default_value_t = 0)]
    secondary_offset: usize,
}

impl PlayCmd {
    pub fn run(&self) -> Result<()> {
        let primary = {
            let mut art = load_with_fallback(&self.file, &self.fallback_file)?;
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
