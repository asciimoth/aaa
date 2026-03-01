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
use std::str::FromStr;

use anyhow::Result;
use rs3a::{Color, ColorPair, chars::Char};

#[derive(clap::Args, PartialEq, Debug)]
pub struct ColorMapCmd {
    /// foreground
    #[clap(long)]
    fg: Option<String>,

    /// background
    #[clap(long)]
    bg: Option<String>,
}

impl ColorMapCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        let fg = if let Some(fg_str) = &self.fg {
            Color::from_str(&fg_str)?
        } else {
            Color::None
        };
        let bg = if let Some(bg_str) = &self.bg {
            Color::from_str(&bg_str)?
        } else {
            Color::None
        };
        let pair = ColorPair { fg, bg };
        eprintln!("{}", art.search_or_create_color_map(pair));
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct ColorUnMapCmd {
    /// mapping name
    #[clap(long)]
    name: char,
}

impl ColorUnMapCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        if let Ok(name) = Char::new(self.name) {
            art.remove_color_map(name);
        }
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct PaletteResetCmd {}

impl PaletteResetCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.remove_palette();
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct ColorForceCmd {
    /// color flag
    #[clap(long)]
    color: bool,
}

impl ColorForceCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_colors_key(Some(self.color));
        Ok(())
    }
}
