use std::str::FromStr;

use anyhow::Result;
use argh::FromArgs;
use rs3a::{art, chars::Char, Color, ColorPair};

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
