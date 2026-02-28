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
use argh::FromArgs;
use rs3a::{Color, ColorPair, chars::Char};

/// Search or add new color mapping.
/// Mapped color prints to stderr.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "color-map")]
pub struct CmdColorMap {
    /// foreground
    #[argh(option)]
    fg: Option<String>,

    /// background
    #[argh(option)]
    bg: Option<String>,
}

impl CmdColorMap {
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

/// Remove color mapping.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "color-unmap")]
pub struct CmdColorUnMap {
    /// mapping name
    #[argh(positional)]
    name: char,
}

impl CmdColorUnMap {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        if let Ok(name) = Char::new(self.name) {
            art.remove_color_map(name);
        }
        Ok(())
    }
}

/// Reset palette.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "palette-reset")]
pub struct CmdPaletteRest {}

impl CmdPaletteRest {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.remove_palette();
        Ok(())
    }
}

/// Force enable/disable colors
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "color-force")]
pub struct CmdColorForce {
    /// color flag
    #[argh(positional)]
    color: bool,
}

impl CmdColorForce {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_colors_key(Some(self.color));
        Ok(())
    }
}
