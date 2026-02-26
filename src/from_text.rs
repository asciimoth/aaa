use std::{
    fs,
    io::{self, Read},
};

use anyhow::Result;
use argh::FromArgs;
use rs3a::Art;

/// Constructs art from plain text with ANSI color escape codes
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "from-text")]
pub struct CmdFromText {
    /// text file path (alternatively pipe text to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// should art be looped
    #[argh(option, short = 'l', long = "loop")]
    loop_flag: Option<bool>,

    /// should colors be enabled
    #[argh(option, short = 'c')]
    color: Option<bool>,

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

    /// art license
    #[argh(option)]
    license: Option<String>,

    /// art tags
    #[argh(option)]
    tag: Vec<String>,
}

// TODO: animation
//       - ruller
//       - line by line
//       - line by line from side
//       - random appear
//       - sand

impl CmdFromText {
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
