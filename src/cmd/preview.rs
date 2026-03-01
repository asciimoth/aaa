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

use crate::loader::load;

/// Show art preview
#[derive(clap::Args, PartialEq, Debug)]
pub struct PreviewCmd {
    /// art file path (alternatively pipe art to stdin)
    file: Option<String>,

    /// disable colors
    #[arg(long, short = 'n')]
    no_colors: bool,

    /// preview frame index
    #[arg(short = 'f', default_value_t = 0)]
    frame: usize,
}

impl PreviewCmd {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }

        let frames = art.to_ansi_frames();

        if frames.len() > 0 {
            println!(
                "{}",
                frames[art
                    .get_preview_key()
                    .unwrap_or(self.frame)
                    .min(frames.len() - 1)]
            );
        } else {
            eprintln!("There is no frames in art")
        };
        Ok(())
    }
}
