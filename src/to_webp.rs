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

use crate::{
    img::{ImgColorMap, ImgFont, render_webp},
    loader::load,
};

/// Convert art to webp animation (ffmpeg cli required)
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-webp")]
pub struct CmdToWebp {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// whether loop aniamtion
    #[argh(option, long = "loop")]
    loop_flag: Option<bool>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// losless encoding
    #[argh(switch, short = 'l')]
    losless: bool,

    /// compression quality 0-100
    #[argh(option, default = "50")]
    quality: usize,

    /// ttf font file
    #[argh(option)]
    font_file: Option<String>,

    /// font size in pixels
    #[argh(option)]
    font_size: Option<f32>,

    /// font cell width
    #[argh(option)]
    font_width: Option<i32>,

    /// font cell height
    #[argh(option)]
    font_height: Option<i32>,

    /// font glyphs x offset
    #[argh(option)]
    glyph_offset_x: Option<i32>,

    /// font glyphs y offset
    #[argh(option)]
    glyph_offset_y: Option<i32>,

    /// define a color mapping like fg:red=ff0000
    #[argh(option, short = 'm')]
    color_map: Vec<String>,

    /// default foreground color
    #[argh(option)]
    fg: Option<String>,

    /// default background color
    #[argh(option)]
    bg: Option<String>,
}

impl CmdToWebp {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }
        if let Some(loop_flag) = self.loop_flag {
            art.set_loop_key(loop_flag);
        }

        render_webp(
            &art,
            &self.to_font()?,
            &self.to_color_map(),
            self.losless,
            self.quality,
        )?;
        Ok(())
    }
    fn to_font(&self) -> Result<ImgFont> {
        let mut font = ImgFont::default();
        if let Some(path) = self.font_file.clone() {
            font.font_from_file(&path)?;
        }
        if let Some(size) = self.font_size {
            font.set_size(size)?;
        }
        if let Some(w) = self.font_width {
            font.cell_w = w;
        }
        if let Some(h) = self.font_height {
            font.cell_h = h;
        }
        if let Some(x) = self.glyph_offset_x {
            font.glyph_x_offset = x;
        }
        if let Some(y) = self.glyph_offset_y {
            font.glyph_y_offset = y;
        }
        Ok(font)
    }
    fn to_color_map(&self) -> ImgColorMap {
        let mut map = ImgColorMap::default();
        if self.color_map.len() > 0 {
            map.set_map(&self.color_map);
        }
        if let Some(fg) = &self.fg {
            map.set_default_fg(fg);
        }
        if let Some(bg) = &self.bg {
            map.set_default_bg(bg);
        }
        map
    }
}
