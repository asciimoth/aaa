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
use rs3a;
use std::io::{stdout, Write};
use crossterm::{
    terminal,
    ExecutableCommand, QueueableCommand,
    cursor, style::{self, Stylize}, self
};
use crossterm::event::{poll, read, Event};
use std::time::Duration;

fn is_bold(color: rs3a::Color) -> bool {
    match color {
        rs3a::Color::GRAY => {true}
        rs3a::Color::BRIGHT_BLUE => {true}
        rs3a::Color::BRIGHT_GREEN => {true}
        rs3a::Color::BRIGHT_CYAN => {true}
        rs3a::Color::BRIGHT_RED => {true}
        rs3a::Color::BRIGHT_MAGENTA => {true}
        rs3a::Color::BRIGHT_YELLOW => {true}
        rs3a::Color::BRIGHT_WHITE => {true}
        _ => {false}
    }
}

fn to_color(color: rs3a::Color) -> style::Color {
    match color {
        rs3a::Color::BLACK => {style::Color::Black}
        rs3a::Color::BLUE => {style::Color::DarkBlue}
        rs3a::Color::GREEN => {style::Color::DarkGreen}
        rs3a::Color::CYAN => {style::Color::DarkCyan}
        rs3a::Color::RED => {style::Color::DarkRed}
        rs3a::Color::MAGENTA => {style::Color::DarkMagenta}
        rs3a::Color::YELLOW => {style::Color::DarkYellow}
        rs3a::Color::WHITE => {style::Color::Grey}
        rs3a::Color::GRAY => {style::Color::DarkGrey}
        rs3a::Color::BRIGHT_BLUE => {style::Color::Blue}
        rs3a::Color::BRIGHT_GREEN => {style::Color::Green}
        rs3a::Color::BRIGHT_CYAN => {style::Color::Cyan}
        rs3a::Color::BRIGHT_RED => {style::Color::Red}
        rs3a::Color::BRIGHT_MAGENTA => {style::Color::Magenta}
        rs3a::Color::BRIGHT_YELLOW => {style::Color::Yellow}
        rs3a::Color::BRIGHT_WHITE => {style::Color::White}
    }
}

pub fn render_fragment(fragment: rs3a::RowFragment) -> String {
    let mut ret = fragment.text;
    if ret.len() > 0 {
        if let Some(fg) = fragment.fg_color {
            ret = format!("{}", ret.with(to_color(fg)));
            if is_bold(fg){
                ret = format!("{}", ret.bold());
            }
        }
        if let Some(bg) = fragment.bg_color {
            ret = format!("{}", ret.on(to_color(bg)));
        }
    }
    ret
}

pub fn render_frame(frame: rs3a::Frame) -> String {
    let mut ret = String::new();
    for row in frame {
        for fragment in row{
            ret += &render_fragment(fragment);
        }
        ret.push('\n');
    }
    ret
}

pub fn render(frames: Vec<rs3a::Frame>) -> Vec<String> {
    let mut ret = Vec::new();
    for frame in frames {
        ret.push(render_frame(frame));
    }
    ret
}

pub fn render_raw_mod(frames: Vec<rs3a::Frame>) -> Vec<Vec<String>> {
    let r = render(frames);
    let mut ret = Vec::new();
    for fr in r {
        let mut frame = Vec::new();
        for s in fr.split("\n"){
            frame.push(s.to_string());
        }
        ret.push(frame);
    }
    ret
}

pub fn get_title(title: Option<String>, author: Option<String>) -> Option<String>{
    if title == None && author == None { return None }
    let mut ret = String::new();
    if let Some(s) = title {
        ret += &s;
    }
    if let Some(s) = author {
        if ret != "" {
            ret = format!("\"{}\" by ", ret);
        }
        ret += &s;
    }
    Some(ret)
}

pub fn optimize(mut art: rs3a::Art) -> rs3a::Art{
    if art.body.frames.len() > 0 {
        let mut new_frames: Vec<rs3a::Frame> = Vec::new();
        let mut last_frame = art.body.frames[0].clone();
        let mut first = true;
        for frame in art.body.frames{
            if first {
                new_frames.push(frame.clone());
                first = false;
            }else{
                let mut new_frame = frame.clone();
                for i in 0..new_frame.len() {
                    if last_frame[i] == new_frame[i] {
                        new_frame[i] = Vec::new();
                    }
                }
                new_frames.push(new_frame);
            }
            last_frame = frame;
        }
        art.body.frames = new_frames;
    }
    art
}

pub fn play(art: rs3a::Art, lx: u16, ly: u16) -> crossterm::Result<()>{
    let rows_count = art.header.height;
    let art = optimize(art);
    let mut stdout = stdout();
    stdout.execute(cursor::Hide)?;
    terminal::enable_raw_mode()?;
    if let Some(title) = get_title(art.header.title, art.header.author) {
        stdout.execute(terminal::SetTitle(title))?;
    }
    for _ in 0..rows_count {
        print!("\n");
    }
    let (_, mut sy) = cursor::position()?;
    sy -= art.header.height;
    sy += ly;
    let frames = render_raw_mod(art.body.frames);
    let d = Duration::from_millis(art.header.delay as u64);
    let l = frames.len()-1;
    'outer: loop {
        for (i, frame) in (&frames).iter().enumerate() {
            for (i, row) in (&frame).iter().enumerate() {
                if row.len() > 0 {
                    stdout
                        .queue(cursor::MoveTo(lx,sy+i as u16))?
                        .queue(style::Print(row))?;
                }
            }
            stdout.flush()?;
            if i >= l {
                if !art.header.loop_enable {break 'outer;}
            }
            if poll(d)? {
                match read()? {
                    Event::Key(_) => {break 'outer;}
                    _ => {}
                }
            }
        }
    }
    stdout.queue(cursor::MoveTo(0,sy+rows_count as u16))?;
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    Ok(())
}
