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
use clap::Args;
use rs3a::{Cell, chars::Char};

#[derive(Args, PartialEq, Debug)]
pub struct GenCmd {
    frames: usize,
    height: usize,
    width: usize,
    text: char,

    /// color value
    #[clap(short = 'c')]
    color: Option<char>,

    /// art title
    #[clap(short = 't')]
    title: Option<String>,

    /// art author
    #[clap(short = 'a')]
    author: Option<String>,

    /// art original author
    #[clap(short = 'o')]
    orig: Option<String>,

    /// art source
    #[clap(short = 's')]
    src: Option<String>,

    /// art editor
    #[clap(short = 'e')]
    edit: Option<String>,

    /// art license
    #[clap(long)]
    license: Option<String>,

    /// should art be looped
    #[clap(short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// art preview frame
    #[clap(short = 'p')]
    preview: Option<usize>,

    /// art tags
    #[clap(long)]
    tag: Vec<String>,
}

impl GenCmd {
    pub fn run(&self) -> Result<()> {
        let text = Char::new(self.text)?;
        let color = if let Some(color) = self.color {
            Some(Char::new(color)?)
        } else {
            None
        };
        let mut art = rs3a::Art::new(self.frames, self.width, self.height, Cell { text, color });
        art.set_title_key(self.title.clone());
        if let Some(author) = &self.author {
            art.set_authors_key(&vec![author.clone()]);
        }
        if let Some(orig) = &self.orig {
            art.set_orig_authors_key(&vec![orig.clone()]);
        }
        art.set_src_key(self.src.clone());
        art.set_editor_key(self.edit.clone());
        art.set_license_key(self.license.clone());
        if let Some(flag) = self.loop_flag {
            art.set_loop_key(flag);
        }
        art.set_preview_key(self.preview);
        for tag in &self.tag {
            art.add_tag(tag);
        }
        print!("{}", art);
        Ok(())
    }
}
