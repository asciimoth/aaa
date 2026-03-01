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
use std::{
    fs,
    io::{self, Read},
};

use anyhow::Result;
use rs3a::Art;

use crate::effects::Effect;

/// Constructs art from plain text with ANSI color escape codes
#[derive(clap::Args, PartialEq, Debug)]
pub struct FromTextCmd {
    /// text file path (alternatively pipe text to stdin)
    #[arg(value_name = "FILE")]
    file: Option<String>,

    /// should art be looped
    #[arg(short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// should colors be enabled (provide a boolean value, e.g. --color true)
    #[arg(short = 'c', long, value_name = "BOOL")]
    color: Option<bool>,

    /// art title
    #[arg(short = 't', long)]
    title: Option<String>,

    /// art author
    #[arg(short = 'a', long)]
    author: Option<String>,

    /// art original author
    #[arg(short = 'o', long)]
    orig: Option<String>,

    /// art source
    #[arg(short = 's', long)]
    src: Option<String>,

    /// art license
    #[arg(long)]
    license: Option<String>,

    /// art tags (can be used multiple times)
    #[arg(long, value_name = "TAG")]
    tag: Vec<String>,

    /// animation effect
    #[arg(long, value_enum, default_value_t = Effect::None)]
    effect: Effect,
}

impl FromTextCmd {
    pub fn run(&self) -> Result<()> {
        let txt = match &self.file {
            Some(path) => fs::read_to_string(path)?,
            None => {
                let mut txt = String::new();
                io::stdin().read_to_string(&mut txt)?;
                txt
            }
        };
        let mut art = Art::from_ansi_text(&txt);
        self.effect.apply(&mut art, 0);
        if let Some(flag) = self.loop_flag {
            art.set_loop_key(flag);
        }
        if let Some(color) = self.color {
            art.set_colors_key(Some(color));
        }
        if let Some(title) = self.title.clone() {
            art.set_title_key(Some(title));
        }
        if let Some(author) = &self.author {
            art.set_authors_key(&vec![author.clone()]);
        }
        if let Some(orig) = &self.orig {
            art.set_orig_authors_key(&vec![orig.clone()]);
        }
        art.set_src_key(self.src.clone());
        art.set_license_key(self.license.clone());
        for tag in &self.tag {
            art.add_tag(tag);
        }
        println!("{}", art);
        Ok(())
    }
}
