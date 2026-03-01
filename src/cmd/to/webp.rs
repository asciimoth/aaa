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
use rs3a::Art;

use crate::img::{ImgColorMap, ImgFont, render_webp};

#[derive(clap::Args, PartialEq, Debug)]
pub struct WebpCmd {
    /// frame to render (preview frame by default)
    #[arg(long, value_name = "FRAME")]
    frame: Option<usize>,

    /// should art be looped
    #[clap(short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// losless encoding
    #[arg(long)]
    losless: bool,

    /// compression quality 0-100
    #[arg(long, default_value = "50")]
    quality: usize,

    /// ttf font file
    #[arg(long, value_name = "FILE")]
    font_file: Option<String>,

    /// font size in pixels
    #[arg(long, value_name = "SIZE")]
    font_size: Option<f32>,

    /// font cell width
    #[arg(long, value_name = "WIDTH")]
    font_width: Option<i32>,

    /// font cell height
    #[arg(long, value_name = "HEIGHT")]
    font_height: Option<i32>,

    /// font glyphs x offset
    #[arg(long, value_name = "X")]
    glyph_offset_x: Option<i32>,

    /// font glyphs y offset
    #[arg(long, value_name = "Y")]
    glyph_offset_y: Option<i32>,

    /// define a color mapping like fg:red=ff0000
    #[arg(short = 'm', long = "color-map", value_name = "MAP")]
    color_map: Vec<String>,

    /// default foreground color
    #[arg(long, value_name = "COLOR")]
    fg: Option<String>,

    /// default background color
    #[arg(long, value_name = "COLOR")]
    bg: Option<String>,
}

impl WebpCmd {
    pub fn run(&self, art: &mut Art) -> Result<()> {
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
