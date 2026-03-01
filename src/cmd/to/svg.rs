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
use rs3a::Art;

#[derive(clap::Args, PartialEq, Debug)]
pub struct SvgCmd {
    /// whether loop animation
    #[clap(short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// font family like `Courier New`
    #[arg(long, value_name = "FAMILY")]
    font_family: Option<String>,

    /// font size in pixels
    #[arg(long, value_name = "SIZE")]
    font_size: Option<usize>,

    /// font cell width
    #[arg(long, value_name = "WIDTH")]
    font_width: Option<usize>,

    /// font cell height
    #[arg(long, value_name = "HEIGHT")]
    font_height: Option<usize>,

    /// font foreground x offset
    #[arg(long, value_name = "X")]
    font_offset_x: Option<usize>,

    /// font foreground y offset
    #[arg(long, value_name = "Y")]
    font_offset_y: Option<usize>,

    /// define a color mapping like fg:red=ff0000
    #[arg(short = 'm', long, value_name = "MAP")]
    color_map: Vec<String>,
}

impl SvgCmd {
    pub fn run(&self, art: &mut Art) -> Result<()> {
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
