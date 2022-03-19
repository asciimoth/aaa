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
mod drawer;
mod hardcode;
use rs3a;
use clap::Parser;
use std::fs;
use std::convert::TryFrom;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Opts {
    /// Left up corner x position
    #[clap(short)]
    x: Option<u16>,
    /// Left up corner y position
    #[clap(short)]
    y: Option<u16>,
    /// Show table of available art colors
    #[clap(long)]
    colortable: bool,
    /// Show demo animation
    #[clap(long)]
    demo: bool,
    file: Option<String>,
    /// Print source code instead of rendering it
    #[clap(short, long)]
    print: bool,
    /// Override delay param
    #[clap(short, long)]
    delay: Option<u16>,
    /// Override loop param
    #[clap(short, long)]
    looped: Option<bool>,
    /// Override colors param
    #[clap(short, long)]
    colors: Option<String>,
    /// Override datacols param
    #[clap(long)]
    datacols: Option<u16>,
    /// Override preview param
    #[clap(long)]
    preview: Option<u16>,
    /// Render to plain text instead of animation (render only preview frame)
    #[clap(long)]
    to_plain_text: bool,
    /// Get parameter width value
    #[clap(long)]
    get_param_width: bool,
    /// Get parameter height value
    #[clap(long)]
    get_param_height: bool,
    /// Get parameter delay value
    #[clap(long)]
    get_param_delay: bool,
    /// Get parameter loop value
    #[clap(long)]
    get_param_loop: bool,
    /// Get parameter colors value
    #[clap(long)]
    get_param_colors: bool,
    /// Get parameter utf8 value
    #[clap(long)]
    get_param_utf8: bool,
    /// Get parameter watacols value
    #[clap(long)]
    get_param_datacols: bool,
    /// Get parameter preview value
    #[clap(long)]
    get_param_preview: bool,
    /// Get parameter audio value
    #[clap(long)]
    get_param_audio: bool,
    /// Get parameter title value
    #[clap(long)]
    get_param_title: bool,
    /// Get parameter author value
    #[clap(long)]
    get_param_author: bool,
    /// Return source code header
    #[clap(long)]
    get_header: bool,
    /// Return source code body, stripped of comments and no display characters
    #[clap(long)]
    get_clear_body: bool,
    /// Return pretifyed source code body
    #[clap(long)]
    get_pretify_body: bool,
    /// Return source code without comments
    #[clap(long)]
    get_escape_comments: bool,
}

fn art_to_plain_text(art: rs3a::Art) -> String {
    drawer::render_frame(art.body.frames[art.header.preview as usize].clone())
}

fn main() {
    let opts: Opts = Opts::parse();
    let content: String;
    if opts.colortable {
        content = hardcode::COLORTABLE.to_string();
    }else if opts.demo {
        content = hardcode::DEMO.to_string();
    }else if let Some(file) = opts.file{
        content = match fs::read_to_string(file){
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error {}", e);
                std::process::exit(exitcode::IOERR);
            }
        };
    }else{
        eprintln!("Error: No content! Use --colortable or --demo or give a file to read");
        std::process::exit(exitcode::DATAERR);
    }
    if opts.print {
        print!("{}", content);
        return;
    }
    if opts.get_escape_comments {
        print!("{}", rs3a::escape_comments(&content));
        return;
    }
    let mut art = match rs3a::load(content){
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
    };
    if let Some(value) = opts.delay{
        art.header.delay = value;
    }
    if let Some(value) = opts.looped{
        art.header.loop_enable = value;
    }
    if let Some(value) = opts.colors{
        art.header.color_mod = match rs3a::ColorMod::try_from(value.as_str()){
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(exitcode::DATAERR);
            }
        };
    }
    if let Some(value) = opts.datacols{
        art.header.datacols = value;
    }
    if let Some(value) = opts.preview{
        art.header.preview = value;
    }
    if opts.to_plain_text {
        print!("{}", art_to_plain_text(art));
        return;
    }
    if opts.get_param_width {
        print!("{}", art.header.width);
        return;
    }
    if opts.get_param_height {
        print!("{}", art.header.height);
        return;
    }
    if opts.get_param_delay {
        print!("{}", art.header.delay);
        return;
    }
    if opts.get_param_loop {
        print!("{}", art.header.loop_enable);
        return;
    }
    if opts.get_param_colors {
        let s: String = art.header.color_mod.into();
        print!("{}", s);
        return;
    }
    if opts.get_param_utf8 {
        print!("{}", art.header.utf8);
        return;
    }
    if opts.get_param_datacols {
        print!("{}", art.header.datacols);
        return;
    }
    if opts.get_param_preview {
        print!("{}", art.header.preview);
        return;
    }
    if opts.get_param_audio {
        if let Some(audio) = art.header.audio{
            print!("{}", audio);
        }
        return;
    }
    if opts.get_param_title {
        if let Some(title) = art.header.title{
            print!("{}", title);
        }
        return;
    }
    if opts.get_param_author {
        if let Some(author) = art.header.author{
            print!("{}", author);
        }
        return;
    }
    if opts.get_clear_body {
        print!("{}", art.body.to_string(false));
        return;
    }
    if opts.get_pretify_body {
        print!("{}", art.body.to_string(true));
        return;
    }
    if opts.get_header {
        let s: String = art.header.into();
        print!("{}", s);
        return;
    }
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    if let Some(lx) = opts.x {x = lx}
    if let Some(ly) = opts.y {y = ly}
    if let Err(e) = drawer::play(art, x, y){
        eprintln!("Error: {}", e);
        std::process::exit(exitcode::UNAVAILABLE);
    }
}
