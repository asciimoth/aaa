use std::{
    fmt::format,
    io::{self, Write},
    str::FromStr,
    sync::mpsc::{channel, Receiver},
    thread,
};

use anyhow::Result;
use argh::FromArgs;
use rs3a::header;

use crate::{
    frames::{art2frames, FrameWithDelay},
    loader::load,
};

/// Convert art to svg
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-svg")]
pub struct CmdToSvg {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// whether loop aniamtion
    #[argh(option, long = "loop")]
    loop_flag: Option<bool>,

    /// font family
    #[argh(option)]
    font_family: Option<String>,

    /// font size in pixels
    #[argh(option)]
    font_size: Option<usize>,

    /// font cell width
    #[argh(option)]
    font_width: Option<usize>,

    /// font cell height
    #[argh(option)]
    font_height: Option<usize>,

    /// font foreground x offset
    #[argh(option)]
    font_offset_x: Option<usize>,

    /// font foreground y offset
    #[argh(option)]
    font_offset_y: Option<usize>,

    /// define a color mapping like fg:red=ff0000
    #[argh(option, short = 'm')]
    color_map: Vec<String>,
}

impl CmdToSvg {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }
        if let Some(loop_flag) = self.loop_flag {
            art.set_loop_key(loop_flag);
        }
        println!(
            "{}",
            art.to_svg_frames(&self.to_color_map(), &self.to_font())
        );
        Ok(())
    }
    fn to_color_map(&self) -> rs3a::CSSColorMap {
        let mut map = rs3a::CSSColorMap::default();
        for pair in &self.color_map {
            if let Some((key, rgb)) = pair.split_once('=') {
                if let Some((g, color)) = key.split_once(':') {
                    let fg = g == "fg";
                    if let Ok(color) = rs3a::Color::from_str(color) {
                        let rgb = format!("#{}", rgb);
                        map.map.insert((color, fg), rgb);
                        continue;
                    }
                }
            }
            eprintln!("Warning: ignoring color mapping '{}'", pair);
        }
        map
    }
    fn to_font(&self) -> rs3a::font::Font {
        let mut font = rs3a::font::Font::default();
        if let Some(family) = self.font_family.clone() {
            font.family = family
        }
        if let Some(size) = self.font_size.clone() {
            font.size = size
        }
        if let Some(width) = self.font_width.clone() {
            font.width = width
        }
        if let Some(height) = self.font_height.clone() {
            font.height = height
        }
        if let Some(x) = self.font_offset_x.clone() {
            font.fg_offset_x = x
        }
        if let Some(y) = self.font_offset_y.clone() {
            font.fg_offset_y = y
        }
        font
    }
}
