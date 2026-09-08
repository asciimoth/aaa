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
use std::{
    fs,
    io::{self, Read},
};

use anyhow::Result;
use rs3a::{Art, TiledTextOptions};

/// Construct art from colorless plain-text frames arranged in a grid
#[derive(clap::Args, PartialEq, Debug)]
pub struct FromTiledTextCmd {
    /// maximum number of tiles in each grid row
    columns: usize,

    /// maximum number of tiles in each grid column
    rows: usize,

    /// text file path (alternatively pipe text to stdin)
    #[arg(value_name = "FILE")]
    file: Option<String>,

    /// exact width of each tile in characters
    #[arg(long, visible_alias = "cell-width")]
    tile_width: Option<usize>,

    /// exact height of each tile in lines
    #[arg(long, visible_alias = "cell-height")]
    tile_height: Option<usize>,

    /// width of the ignored gap between tiles
    #[arg(long)]
    horizontal_gap: Option<usize>,

    /// height of the ignored gap between tile rows
    #[arg(long)]
    vertical_gap: Option<usize>,
}

impl FromTiledTextCmd {
    pub fn run(&self) -> Result<()> {
        let text = match &self.file {
            Some(path) => fs::read_to_string(path)?,
            None => {
                let mut text = String::new();
                io::stdin().read_to_string(&mut text)?;
                text
            }
        };
        let options = TiledTextOptions {
            columns: self.columns,
            rows: self.rows,
            cell_width: self.tile_width,
            cell_height: self.tile_height,
            horizontal_gap: self.horizontal_gap,
            vertical_gap: self.vertical_gap,
        };
        let art = Art::from_tiled_text(&text, options)?;
        print!("{}", art);
        Ok(())
    }
}
