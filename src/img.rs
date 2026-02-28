use std::collections::HashMap;
use std::fs::{self, File};
use std::io::stdout;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::Duration;

use ab_glyph::{FontArc, PxScale};
use anyhow::{Result, anyhow};
use image::{Frame, Rgba, RgbaImage, codecs::gif::GifEncoder};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rs3a::{Art, Color, Color4};
use std::io::copy;
use tempfile::TempDir;

#[derive(Debug, Clone)]
pub struct ImgColorMap {
    // Color, fg
    pub map: HashMap<(Color, bool), Rgba<u8>>,
    pub default_fg: Rgba<u8>,
    pub default_bg: Rgba<u8>,
}

impl Default for ImgColorMap {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
            default_fg: Rgba([230, 230, 230, 255]),
            default_bg: Rgba([0, 0, 0, 255]),
        }
    }
}

impl ImgColorMap {
    // args: "fg:red=ff0000"
    pub fn set_map(&mut self, args: &[String]) {
        let mut map = HashMap::new();
        for pair in args {
            if let Some((key, rgb)) = pair.split_once('=') {
                if let Some((g, color)) = key.split_once(':') {
                    let fg = g == "fg";
                    if let Ok(color) = rs3a::Color::from_str(color) {
                        if let Ok(rgb) = rgba_from_hex(rgb) {
                            map.insert((color, fg), rgb);
                        }
                        continue;
                    }
                }
            }
            eprintln!("Warning: ignoring invalid color mapping '{}'", pair);
        }
        self.map = map;
    }

    pub fn set_default_fg(&mut self, fg: &str) {
        if let Ok(fg) = rgba_from_hex(fg) {
            self.default_fg = fg;
        } else {
            eprintln!("Warning: ignoring invalid color '{}'", fg);
        }
    }

    pub fn set_default_bg(&mut self, bg: &str) {
        if let Ok(bg) = rgba_from_hex(bg) {
            self.default_bg = bg;
        } else {
            eprintln!("Warning: ignoring invalid color '{}'", bg);
        }
    }

    pub fn color_to_rgba(&self, c: Color, fg: bool) -> Rgba<u8> {
        if let Some(rgba) = self.map.get(&(c, fg)) {
            return *rgba;
        }
        match c {
            Color::None => {
                if fg {
                    self.default_fg
                } else {
                    self.default_bg
                }
            }
            Color::RGB(r, g, b) => Rgba([r, g, b, 255]),
            Color::Color256(c) => {
                let c = c as usize;
                // first 16 are the standard/system colors
                let table16 = [
                    "#000000", "#800000", "#008000", "#808000", "#000080", "#800080", "#008080",
                    "#c0c0c0", "#4e4e4e", "#ff0000", "#00ff00", "#ffff00", "#0000ff", "#ff00ff",
                    "#00ffff", "#ffffff",
                ];
                let code = if c < 16 {
                    table16[c].to_string()
                } else if c < 232 {
                    // 6x6x6 color cube
                    let idx = c - 16;
                    let r = idx / 36;
                    let g = (idx % 36) / 6;
                    let b = idx % 6;
                    let levels: [u8; 6] = [0, 95, 135, 175, 215, 255];
                    format!("#{:02x}{:02x}{:02x}", levels[r], levels[g], levels[b])
                } else {
                    // grayscale ramp: 232..255 -> 24 shades
                    let gray = 8 + (c - 232) * 10;
                    format!("#{:02x}{:02x}{:02x}", gray, gray, gray)
                };
                rgba_from_hex(&code).unwrap()
            }
            Color::Color4(c, b) => match (c, b) {
                (Color4::Black, true) => Rgba([102, 102, 102, 255]),
                (Color4::Black, false) => Rgba([0, 0, 0, 255]),
                (Color4::Red, true) => Rgba([230, 0, 0, 255]),
                (Color4::Red, false) => Rgba([153, 0, 0, 255]),
                (Color4::Green, true) => Rgba([0, 217, 0, 255]),
                (Color4::Green, false) => Rgba([0, 166, 0, 255]),
                (Color4::Yellow, true) => Rgba([230, 230, 0, 255]),
                (Color4::Yellow, false) => Rgba([153, 153, 0, 255]),
                (Color4::Blue, true) => Rgba([0, 0, 255, 255]),
                (Color4::Blue, false) => Rgba([0, 0, 178, 255]),
                (Color4::Magenta, true) => Rgba([230, 0, 230, 255]),
                (Color4::Magenta, false) => Rgba([178, 0, 178, 255]),
                (Color4::Cyan, true) => Rgba([0, 230, 230, 255]),
                (Color4::Cyan, false) => Rgba([0, 166, 178, 255]),
                (Color4::White, true) => Rgba([230, 230, 230, 255]),
                (Color4::White, false) => Rgba([191, 191, 191, 255]),
            },
        }
    }
}

pub fn rgba_from_hex(s: &str) -> Result<Rgba<u8>> {
    let s = s.trim();

    // Remove optional leading '#'
    let hex = if let Some(stripped) = s.strip_prefix('#') {
        stripped
    } else {
        s
    };

    if hex.len() != 6 {
        return Err(anyhow!("Hex color must be 6 characters (RRGGBB)"));
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| anyhow!("Invalid red component"))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| anyhow!("Invalid green component"))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| anyhow!("Invalid blue component"))?;

    Ok(Rgba([r, g, b, 255]))
}

#[derive(Debug, Clone)]
pub struct ImgFont {
    pub font: FontArc,
    pub scale: PxScale,
    pub cell_w: i32,
    pub cell_h: i32,
    pub glyph_y_offset: i32,
    pub glyph_x_offset: i32,
}

impl ImgFont {
    pub fn font_from_file(&mut self, path: &str) -> Result<()> {
        self.font = FontArc::try_from_vec(fs::read(path)?)?;
        Ok(())
    }
    pub fn set_size(&mut self, size: f32) -> Result<()> {
        self.scale = PxScale::from(size);
        self.cell_w = (self.scale.x * 0.6).ceil() as i32;
        self.cell_h = (self.scale.y * 1.2).ceil() as i32;
        Ok(())
    }
}

impl Default for ImgFont {
    fn default() -> Self {
        // let font_bytes = include_bytes!("../DejaVuSansMono.ttf").to_vec();
        let font_bytes = include_bytes!("../AdwaitaMonoNerdFontMono-Regular.ttf").to_vec();
        let scale = PxScale::from(25.0);

        Self {
            glyph_y_offset: 0,
            glyph_x_offset: 0,
            font: FontArc::try_from_vec(font_bytes).unwrap(),
            scale: scale,
            cell_w: (scale.x * 0.5).ceil() as i32,
            cell_h: (scale.y * 1.1).ceil() as i32,
        }
    }
}

fn render_with_ffmpeg(
    art: &Art,
    font: &ImgFont,
    col: &ImgColorMap,
    mut cmd: Vec<String>,
    tmpout: Option<&str>,
) -> Result<()> {
    let tmp = TempDir::new()?;
    let tmp_path = tmp.path();
    let list_path = tmp_path.join("list.txt");
    let mut out_path = tmp_path.join("tmp.tmp");

    eprintln!("rendering frames:");
    {
        let mut contents = String::new();
        let mut last_name = String::new();
        for f in 0..art.frames() {
            eprintln!("  {}/{}", f + 1, art.frames());
            let frame = render_frame(&art, f, font, col);
            let delay = art.get_frame_delay(f) as f64 / 1000.0;
            let name = format!("frame_{:010}.png", f);
            let dst = tmp_path.join(&name);
            frame.save(dst)?;
            contents.push_str(&format!("file '{}'\n", name));
            contents.push_str(&format!("duration {:.3}\n", delay));
            last_name = name;
        }
        contents.push_str(&format!("file '{}'\n", last_name));
        fs::write(&list_path, contents)?;
    }

    cmd.insert(0, String::from(list_path.to_str().unwrap()));
    cmd.insert(0, String::from("-i"));
    cmd.insert(0, String::from("0"));
    cmd.insert(0, String::from("-safe"));
    cmd.insert(0, String::from("concat"));
    cmd.insert(0, String::from("-f"));
    cmd.insert(0, String::from("-y"));

    if let Some(tmpout) = tmpout {
        out_path = tmp_path.join(tmpout);
        cmd.push(String::from(out_path.to_str().unwrap()));
    }

    // eprintln!("{:?}", cmd);

    eprintln!("encoding animation:");
    // run ffmpeg to write the file (not to pipe)
    let status = Command::new("ffmpeg")
        .current_dir(tmp_path)
        .args(cmd)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if !status.success() {
        anyhow::bail!("ffmpeg failed (exit {})", status);
    }

    if tmpout != None {
        let mut f = File::open(&out_path)?;
        copy(&mut f, &mut stdout())?;
    }

    Ok(())
}

pub fn render_mp4(
    art: &Art,
    font: &ImgFont,
    col: &ImgColorMap,
    preset: &str,
    crf: usize,
) -> Result<()> {
    let crf = crf.to_string();
    let cmd = [
        "-vf",
        "scale=ceil(iw/2)*2:ceil(ih/2)*2",
        "-vsync",
        "vfr",
        "-pix_fmt",
        "yuv420p",
        "-c:v",
        "libx264",
        "-preset",
        preset, // e.g. "medium"
        "-crf",
        &crf, // e.g. 18
        "-movflags",
        "+faststart+frag_keyframe+empty_moov",
        "-f",
        "mp4",
        "pipe:1",
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect();
    render_with_ffmpeg(art, font, col, cmd, None)
}

pub fn render_webp(
    art: &Art,
    font: &ImgFont,
    col: &ImgColorMap,
    losless: bool,
    quality: usize,
) -> Result<()> {
    let mut cmd: Vec<String> = [
        "-vf",
        "scale=ceil(iw/2)*2:ceil(ih/2)*2",
        "-vsync",
        "vfr",
        "-pix_fmt",
        "rgba",
        "-c:v",
        "libwebp",
        "-lossless",
        if losless { "1" } else { "0" },
        "-q:v",
        &quality.to_string(), // e.g. 50
    ]
    .iter()
    .map(|s| String::from(*s))
    .collect();
    if art.get_loop_key() {
        cmd.push(String::from("-loop"));
        cmd.push(String::from("0"));
    }
    render_with_ffmpeg(art, font, col, cmd, Some("art.webp"))
}

pub fn render_gif(art: &Art, font: &ImgFont, col: &ImgColorMap) -> image::ImageResult<()> {
    // let file = File::create(path)?;

    let mut encoder = GifEncoder::new(stdout());
    if art.get_loop_key() {
        encoder.set_repeat(image::codecs::gif::Repeat::Infinite)?;
    }

    eprintln!("rendering frames:");
    for frame in 0..art.frames() {
        eprintln!(" {}/{}", frame + 1, art.frames());
        let frame = Frame::from_parts(
            render_frame(&art, frame, font, col),
            0,
            0,
            image::Delay::from_saturating_duration(Duration::from_millis(
                art.get_frame_delay(frame) as u64,
            )),
        );
        encoder.encode_frame(frame)?;
    }
    Ok(())
}

pub fn render_frame(art: &Art, frame: usize, font: &ImgFont, col: &ImgColorMap) -> RgbaImage {
    // compute final image size
    let width = (art.width() as u32).saturating_mul(font.cell_w as u32);
    let height = (art.height() as u32).saturating_mul(font.cell_h as u32);

    let mut img = RgbaImage::from_pixel(width, height, col.default_bg);

    for r in 0..art.height() {
        for c in 0..art.width() {
            let x = (c as i32) * font.cell_w;
            let y = (r as i32) * font.cell_h;
            let cell = art.get(frame, c, r, rs3a::Cell::default());
            if art.color()
                && let Some(name) = cell.color
            {
                let pair = art.get_color_map(name);
                let bg_rgba = col.color_to_rgba(pair.bg, false);
                if bg_rgba != col.default_bg {
                    let rect = Rect::at(x, y).of_size(font.cell_w as u32, font.cell_h as u32);
                    draw_filled_rect_mut(&mut img, rect, bg_rgba);
                }
                let fg_rgba = col.color_to_rgba(pair.fg, true);
                draw_text_mut(
                    &mut img,
                    fg_rgba,
                    x + font.glyph_x_offset,
                    y + font.glyph_y_offset,
                    font.scale,
                    &font.font,
                    &cell.text.to_string(),
                );
            } else {
                // Draw text in default colors
                draw_text_mut(
                    &mut img,
                    col.default_fg,
                    x + font.glyph_x_offset,
                    y + font.glyph_y_offset,
                    font.scale,
                    &font.font,
                    &cell.text.to_string(),
                );
            }
        }
    }

    img
}
