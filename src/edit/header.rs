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

/// Set art title
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "title")]
pub struct CmdTitle {
    #[argh(positional)]
    title: Option<String>,
}

impl CmdTitle {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_title_key(self.title.clone());
        Ok(())
    }
}

/// Set authors
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "authors")]
pub struct CmdAuthors {
    #[argh(positional)]
    authors: Vec<String>,
}

impl CmdAuthors {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_authors_key(&self.authors);
        Ok(())
    }
}

/// Set orig authors
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "origs")]
pub struct CmdOrigs {
    #[argh(positional)]
    authors: Vec<String>,
}

impl CmdOrigs {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_orig_authors_key(&self.authors);
        Ok(())
    }
}

/// Set src
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "src")]
pub struct CmdSrc {
    #[argh(positional)]
    src: Option<String>,
}

impl CmdSrc {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_src_key(self.src.clone());
        Ok(())
    }
}

/// Set editor
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "editor")]
pub struct CmdEditor {
    #[argh(positional)]
    editor: Option<String>,
}

impl CmdEditor {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_editor_key(self.editor.clone());
        Ok(())
    }
}

/// Set license
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "license")]
pub struct CmdLicense {
    #[argh(positional)]
    license: Option<String>,
}

impl CmdLicense {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_license_key(self.license.clone());
        Ok(())
    }
}

/// Set loop
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "loop")]
pub struct CmdLoop {
    #[argh(positional, long = "loop")]
    loop_flag: bool,
}

impl CmdLoop {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_loop_key(self.loop_flag);
        Ok(())
    }
}

/// Set preview frame
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "preview")]
pub struct CmdPreview {
    #[argh(positional)]
    preview: Option<usize>,
}

impl CmdPreview {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_preview_key(self.preview);
        Ok(())
    }
}
