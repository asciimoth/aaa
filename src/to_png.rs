use std::io::{self, Cursor, Write};

use anyhow::Result;
use argh::FromArgs;

use crate::{
    img::{ImgColorMap, ImgFont, render_frame},
    loader::load,
};

/// Convert art to png
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-png")]
pub struct CmdToPng {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// frame to render (preview frame by default)
    #[argh(option)]
    frame: Option<usize>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

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

impl CmdToPng {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }
        if let Some(frame) = self.frame {
            art.set_preview_key(Some(frame));
        }

        let frame = render_frame(
            &art,
            art.get_preview_key().unwrap_or(0),
            &self.to_font()?,
            &self.to_color_map(),
        );

        let mut buf: Vec<u8> = Vec::new();
        let mut cursor = Cursor::new(&mut buf);

        frame.write_to(&mut cursor, image::ImageFormat::Png)?;

        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(&buf)?;
        handle.flush().ok();

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
            map.set_default_fg(bg);
        }
        map
    }
}
