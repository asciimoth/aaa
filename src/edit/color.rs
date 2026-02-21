use std::str::FromStr;

use anyhow::Result;
use argh::FromArgs;
use rs3a::{Color, ColorPair};

/// Search or add new color mapping.
/// Mapped color prints to stderr.
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "color")]
pub struct CmdColor {
    /// foreground
    #[argh(option)]
    fg: Option<String>,

    /// background
    #[argh(option)]
    bg: Option<String>,
}

impl CmdColor {
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
