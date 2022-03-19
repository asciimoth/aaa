/*
    This file is part of rs3a.

    rs3a is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Foobar is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with rs3a.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::convert::{TryFrom, Into};
use std::cmp::PartialEq;
use regex::Regex;
use std::{fs, io, fmt};
#[cfg(test)]
mod tests;

const DEFAULT_DELAY: u16 = 50;
const DEFAULT_PREVIEW: u16 = 0;
const DEFAULT_LOOP: bool = true;
const DEFAULT_COLORS: ColorMod = ColorMod::None;
const DEFAULT_UTF8: bool = false;

#[derive(Debug, Clone)]
pub enum ParcingError{
    UnknownColor(char),
    UnknownColorMod(String),
    InvalidWidth,
    InvalidHeight,
    ThereIsNoBody,
}

impl fmt::Display for ParcingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Self::UnknownColor(c) => {write!(f, "UnknownColor: {}", c)}
            Self::UnknownColorMod(s) => {write!(f, "UnknownColorMod: {}", s)}
            Self::InvalidWidth => {write!(f, "InvalidWidth")}
            Self::InvalidHeight => {write!(f, "InvalidHeight")}
            Self::ThereIsNoBody => {write!(f, "There is no body found")}
        }
    }
}

#[derive(Debug)]
pub enum ReadingError{
    ParcingError(ParcingError),
    IOError(io::Error),
}

impl fmt::Display for ReadingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            Self::ParcingError(e) => {e.fmt(f)}
            Self::IOError(e) => {e.fmt(f)}
        }
    }
}

pub fn escape_comments(s: &str) -> String{
    let re1 = Regex::new(r"(?m)^\t.*?(\n|$)").unwrap();
    let re2 = Regex::new(r"\t.*?(\n|$)").unwrap();
    let s = re1.replace_all(s, "").to_string();
    let s = re2.replace_all(&s, "\n").to_string();
    s
}

pub fn load(s: String) -> Result<Art, ParcingError>{
    let s = escape_comments(&s);
    let fragments: Vec<&str> = s.splitn(2, "\n\n").collect();
    if fragments.len() < 2 {
        return Err(ParcingError::ThereIsNoBody);
    }
    let header: Header = match Header::try_from(fragments[0].to_string()){
        Ok(v) => {v}
        Err(e) => {return Err(e);}
    };
    let body: Body = match Body::from_string(fragments[1].to_string(), header.clone()){
            Ok(v) => {v}
            Err(e) => {return Err(e);}
    };
    Ok(Art{header, body})
}

pub fn save(art: Art, pretify: bool) -> String{
    let mut ret: String = art.header.into();
    ret += "\n";
    ret += &art.body.to_string(pretify);
    ret
}

pub fn load_file(path: String) -> Result<Art, ReadingError>{
    let s = match fs::read_to_string(path) {
        Ok(s) => {s}
        Err(ie) => {return Err(ReadingError::IOError(ie))}
    };
    match load(s) {
        Ok(v) => {Ok(v)}
        Err(e) => {Err(ReadingError::ParcingError(e))}
    }
}

pub fn save_file(art: Art, path: String, pretify: bool) -> Result<(), io::Error>{
    let s = save(art, pretify);
    match fs::write(path, s) {
        Ok(_) => {Ok(())}
        Err(e) => {Err(e)}
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Color{
    BLACK,
    BLUE,
    GREEN,
    CYAN,
    RED,
    MAGENTA,
    YELLOW,
    WHITE,
    GRAY,
    BRIGHT_BLUE,
    BRIGHT_GREEN,
    BRIGHT_CYAN,
    BRIGHT_RED,
    BRIGHT_MAGENTA,
    BRIGHT_YELLOW,
    BRIGHT_WHITE
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowFragment{
    pub text: String,
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
}

pub type Row = Vec<RowFragment>;

pub type Frame = Vec<Row>;

#[derive(Debug, Clone, PartialEq)]
pub struct Body{
    pub frames: Vec<Frame>
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ColorMod{
    None,
    Fg,
    Bg,
    Full
}

#[derive(Debug, Clone, PartialEq)]
pub struct Header{
    pub width: u16,
    pub height: u16,
    pub delay: u16,
    pub loop_enable: bool,
    pub color_mod: ColorMod,
    pub utf8: bool,
    pub datacols: u16,
    pub preview: u16,
    pub audio: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Art{
    pub header: Header,
    pub body: Body,
}

impl Body{
    pub fn from_string(s: String, h: Header) -> Result<Self, ParcingError> {
        let re = Regex::new(r"(\n|\t)").unwrap();
        let s = re.replace_all(&s, "").to_string();
        let char_vec: Vec<char> = s.chars().collect();
        let len = char_vec.len();
        let mut frm: usize = 0; //frame nom
        let width = h.width as usize;
        let height = h.height as usize;
        let datacols = h.datacols as usize;
        let mut frames: Vec<Frame> = Vec::new();
        let mut next = true;
        'outer: while next {
            let mut frame: Frame = Vec::new();
            for y in 0..height{
                let mut row: Row = Vec::new();
                let mut row_fragment = RowFragment{
                    text: "".to_string(),
                    fg_color: None,
                    bg_color: None,
                };
                for x in 0..width{
                    let symbol_pos: usize = (frm*width*datacols*height)+(y*width*datacols)+x;
                    if symbol_pos >= len{
                        next = false;
                        break;
                    }
                    let symbol: char = char_vec[symbol_pos];
                    let mut fg_color: Option<Color> = None;
                    let mut bg_color: Option<Color> = None;
                    match h.color_mod {
                        ColorMod::None => {}
                        ColorMod::Fg => {
                            let fg_color_position = (frm*width*datacols*height)+(y*width*datacols)+width+x;
                            if fg_color_position >= len{
                                next = false;
                                break;
                            }
                            fg_color = Some(match Color::try_from(char_vec[fg_color_position]) {
                                Ok(c) => {c}
                                Err(e) => {return Err(e);}
                            });
                        }
                        ColorMod::Bg => {
                            let bg_color_position = (frm*width*datacols*height)+(y*width*datacols)+width+x;
                            if bg_color_position >= len{
                                next = false;
                                break;
                            }
                            bg_color = Some(match Color::try_from(char_vec[bg_color_position]) {
                                Ok(c) => {c}
                                Err(e) => {return Err(e);}
                            });
                        }
                        ColorMod::Full => {
                            let fg_color_position = (frm*width*datacols*height)+(y*width*datacols)+width+x;
                            let bg_color_position = (frm*width*datacols*height)+(y*width*datacols)+width*2+x;
                            if fg_color_position >= len || bg_color_position >= len {
                                next = false;
                                break;
                            }
                            fg_color = Some(match Color::try_from(char_vec[fg_color_position]) {
                                Ok(c) => {c}
                                Err(e) => {return Err(e);}
                            });
                            bg_color = Some(match Color::try_from(char_vec[bg_color_position]) {
                                Ok(c) => {c}
                                Err(e) => {return Err(e);}
                            });
                        }
                    }
                    if x == 0 {
                        row_fragment.fg_color = fg_color;
                        row_fragment.bg_color = bg_color;
                    }else{
                        if row_fragment.fg_color != fg_color || row_fragment.bg_color != bg_color {
                            row.push(row_fragment);
                            row_fragment = RowFragment{
                                text: symbol.to_string(),
                                fg_color: fg_color,
                                bg_color: bg_color,
                            };
                            continue;
                        }
                    }
                    row_fragment.text.push(symbol)
                }
                if row_fragment.text.len() > 0 {
                    row.push(row_fragment);
                }
                if row.len() < 1 {
                    break 'outer;
                }
                frame.push(row);
            }
            frames.push(frame);
            frm += 1;
        }
        Ok(Body{frames})
    }
    fn generate_color_fragment(color: Option<Color>, count: usize) -> String{
        let mut ret: String = "".to_string();
        if let Some(color) = color {
            let c: char = color.into();
            for _ in 0..count {
                ret.push(c);
            }
        }
        ret
    }
    pub fn to_string(self, pretify: bool) -> String{
        let mut ret: String = "".to_string();
        for (frm, frame) in self.frames.iter().enumerate(){
            for row in frame{
                let mut text_col: String = "".to_string();
                let mut color1_col: String = "".to_string();
                let mut color2_col: String = "".to_string();
                for fragment in row{
                    text_col += &fragment.text;
                    color1_col += &Self::generate_color_fragment(fragment.fg_color, fragment.text.len());
                    color2_col += &Self::generate_color_fragment(fragment.bg_color, fragment.text.len());
                }
                ret += &text_col;
                ret += &color1_col;
                ret += &color2_col;
                if pretify {
                    ret.push('\n');
                }
            }
            if frm < self.frames.len()-1 {
                ret.push('\n');
            }
        }
        ret
    }
}

impl ColorMod{
    fn to_datacols(self) -> u16{
        match self{
            ColorMod::None => {1}
            ColorMod::Fg => {2}
            ColorMod::Bg => {2}
            ColorMod::Full => {3}
        }
    }
}

impl TryFrom<&str> for ColorMod{
    type Error = ParcingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value{
            "none" => {Ok(Self::None)}
            "fg" => {Ok(Self::Fg)}
            "bg" => {Ok(Self::Bg)}
            "full" => {Ok(Self::Full)}
            _ => {Err(ParcingError::UnknownColorMod(value.to_string()))}
        }
    }
}

impl Into<String> for ColorMod{
    fn into(self) -> String{
        match self{
            ColorMod::None => {"none"}
            ColorMod::Fg => {"fg"}
            ColorMod::Bg => {"bg"}
            ColorMod::Full => {"full"}
        }.to_string()
    }
}

impl TryFrom<char> for Color{
    type Error = ParcingError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value{
            '0' => {Ok(Self::BLACK)}
            '1' => {Ok(Self::BLUE)}
            '2' => {Ok(Self::GREEN)}
            '3' => {Ok(Self::CYAN)}
            '4' => {Ok(Self::RED)}
            '5' => {Ok(Self::MAGENTA)}
            '6' => {Ok(Self::YELLOW)}
            '7' => {Ok(Self::WHITE)}
            '8' => {Ok(Self::GRAY)}
            '9' => {Ok(Self::BRIGHT_BLUE)}
            'a' => {Ok(Self::BRIGHT_GREEN)}
            'b' => {Ok(Self::BRIGHT_CYAN)}
            'c' => {Ok(Self::BRIGHT_RED)}
            'd' => {Ok(Self::BRIGHT_MAGENTA)}
            'e' => {Ok(Self::BRIGHT_YELLOW)}
            'f'=> {Ok(Self::BRIGHT_WHITE)}
            _ => {Err(ParcingError::UnknownColor(value))}
        }
    }
}

impl Into<char> for Color{
    fn into(self) -> char{
        match self{
            Self::BLACK             => {'0'}
            Self::BLUE              => {'1'}
            Self::GREEN             => {'2'}
            Self::CYAN              => {'3'}
            Self::RED               => {'4'}
            Self::MAGENTA           => {'5'}
            Self::YELLOW            => {'6'}
            Self::WHITE             => {'7'}
            Self::GRAY              => {'8'}
            Self::BRIGHT_BLUE       => {'9'}
            Self::BRIGHT_GREEN      => {'a'}
            Self::BRIGHT_CYAN       => {'b'}
            Self::BRIGHT_RED        => {'c'}
            Self::BRIGHT_MAGENTA    => {'d'}
            Self::BRIGHT_YELLOW     => {'e'}
            Self::BRIGHT_WHITE      => {'f'}
        }
    }
}

fn only_payload(v: Vec<&str>) -> Vec<&str>{
    let mut ret: Vec<&str> = Vec::new();
    for s in v {
       if s != ""{
        ret.push(s);
       }
    }
    ret
}


impl TryFrom<String> for Header{
    type Error = ParcingError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut width: u16 = 0; let mut w_set = false;
        let mut height: u16 = 0; let mut h_set = false;
        let mut delay: u16 = DEFAULT_DELAY;
        let mut loop_enable: bool = DEFAULT_LOOP;
        let mut color_mod: ColorMod = DEFAULT_COLORS;
        let mut utf8: bool = DEFAULT_UTF8;
        let mut datacols: u16 = 0; let mut d_set = false;
        let mut preview: u16 = DEFAULT_PREVIEW;
        let mut audio: Option<String> = None;
        let mut title: Option<String> = None;
        let mut author: Option<String> = None;
        let rows = s.split("\n").collect::<Vec<&str>>();
        for row in rows{
            let tokens = row.split(" ").collect::<Vec<&str>>();
            if tokens.len() < 1 {continue;}
            let tokens = only_payload(tokens);
            match tokens[0]{
                "width" => {
                    if tokens.len() < 2 {continue;}
                    width = match tokens[1].parse::<u16>(){
                        Ok(v) => {v}
                        Err(_) => {continue;}
                    };
                    w_set = true;
                }
                "height" => {
                    if tokens.len() < 2 {continue;}
                    height = match tokens[1].parse::<u16>(){
                        Ok(v) => {v}
                        Err(_) => {continue;}
                    };
                    h_set = true;
                }
                "delay" => {
                    if tokens.len() < 2 {continue;}
                    delay = match tokens[1].parse::<u16>(){
                        Ok(v) => {v}
                        Err(_) => {continue;}
                    };
                }
                "loop" => {
                    if tokens.len() < 2 {continue;}
                    loop_enable = match tokens[1] {
                        "true" => {true}
                        "false" => {false}
                        _ => {continue;}
                    };
                }
                "colors" => {
                    if tokens.len() < 2 {continue;}
                    color_mod = match ColorMod::try_from(tokens[1]){
                        Ok(v) => {v}
                        _ => {continue;}
                    };
                }
                "utf8" => {
                    utf8 = true;
                }
                "datacols" => {
                    if tokens.len() < 2 {continue;}
                    datacols = match tokens[1].parse::<u16>(){
                        Ok(v) => {v}
                        Err(_) => {continue;}
                    };
                    d_set = true;
                }
                "preview" => {
                    if tokens.len() < 2 {continue;}
                    preview = match tokens[1].parse::<u16>(){
                        Ok(v) => {v}
                        Err(_) => {continue;}
                    };
                }
                "audio" => {
                    if tokens.len() < 2 {continue;}
                    audio = match tokens[1]{
                        "" => {continue;}
                        a => {Some(a.to_string())}
                    };
                }
                "title" => {
                    if tokens.len() < 2 {continue;}
                    let mut s = "".to_string();
                    for i in 1..tokens.len() {
                        if i > 1 {s.push_str(" ")}
                        s.push_str(tokens[i])
                    }
                    title = Some(s);
                }
                "author" => {
                    if tokens.len() < 2 {continue;}
                    let mut s = "".to_string();
                    for i in 1..tokens.len() {
                        if i > 1 {s.push_str(" ")}
                        s.push_str(tokens[i])
                    }
                    author = Some(s);
                }
                _ => {}
            }
        }
        if !w_set {return Err(ParcingError::InvalidWidth);}
        if !h_set {return Err(ParcingError::InvalidHeight);}
        if !d_set {
            datacols = color_mod.to_datacols();
        }
        Ok(Self{width, height, delay, loop_enable, color_mod, utf8, datacols, preview, audio, title, author})
    }
}

impl Into<String> for Header{
    fn into(self) -> String{
        let mut ret = "".to_string();
        ret.push_str("width ");
        ret.push_str(&self.width.to_string());
        ret.push_str("\nheight ");
        ret.push_str(&self.height.to_string());
        if self.delay != DEFAULT_DELAY {
            ret.push_str("\ndelay ");
            ret.push_str(&self.delay.to_string());
        }
        if self.loop_enable != DEFAULT_LOOP {
            ret.push_str("\nloop ");
            ret.push_str(match self.loop_enable{
                true => {"true"}
                false => {"false"}
            });
        }
        if self.color_mod != DEFAULT_COLORS {
            ret.push_str("\ncolors ");
            let s: String = self.color_mod.into();
            ret.push_str(&s);
        }
        if self.utf8 {
            ret.push_str("\nutf8");
        }
        if self.color_mod.to_datacols() != self.datacols{
            ret.push_str("\ndatacols ");
            ret.push_str(&self.datacols.to_string());
        }
        if self.preview != DEFAULT_PREVIEW {
            ret.push_str("\npreview ");
            ret.push_str(&self.preview.to_string());
        }
        if let Some(a) = self.audio {
            ret.push_str("\naudio ");
            ret.push_str(&a);
        }
        if let Some(a) = self.title {
            ret.push_str("\ntitle ");
            ret.push_str(&a);
        }
        if let Some(a) = self.author {
            ret.push_str("\nauthor ");
            ret.push_str(&a);
        }
        ret.push_str("\n\n");
        ret
    }
}
