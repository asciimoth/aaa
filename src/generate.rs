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
use rs3a::{Cell, chars::Char};

/// Generate new art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "generate")]
pub struct CmdGen {
    #[argh(positional)]
    frames: usize,

    #[argh(positional)]
    height: usize,

    #[argh(positional)]
    width: usize,

    #[argh(positional)]
    text: char,

    /// color value
    #[argh(option, short = 'c')]
    color: Option<char>,

    /// art title
    #[argh(option, short = 't')]
    title: Option<String>,

    /// art author
    #[argh(option, short = 'a')]
    author: Option<String>,

    /// art original author
    #[argh(option, short = 'o')]
    orig: Option<String>,

    /// art source
    #[argh(option, short = 's')]
    src: Option<String>,

    /// art editor
    #[argh(option, short = 'e')]
    edit: Option<String>,

    /// art license
    #[argh(option)]
    license: Option<String>,

    /// should art be looped
    #[argh(option, short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// art preview
    #[argh(option, short = 'p')]
    preview: Option<usize>,

    /// art tags
    #[argh(option)]
    tag: Vec<String>,
}

impl CmdGen {
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
