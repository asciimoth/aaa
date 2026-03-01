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

#[derive(clap::Args, PartialEq, Debug)]
pub struct TitleCmd {
    title: Option<String>,
}

impl TitleCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_title_key(self.title.clone());
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct AuthorsCmd {
    authors: Vec<String>,
}

impl AuthorsCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_authors_key(&self.authors);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct OrigsCmd {
    authors: Vec<String>,
}

impl OrigsCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_orig_authors_key(&self.authors);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct SrcCmd {
    src: Option<String>,
}

impl SrcCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_src_key(self.src.clone());
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct EditorCmd {
    editor: Option<String>,
}

impl EditorCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_editor_key(self.editor.clone());
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct LicenseCmd {
    license: Option<String>,
}

impl LicenseCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_license_key(self.license.clone());
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct LoopCmd {
    #[clap(long = "loop")]
    loop_flag: bool,
}

impl LoopCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_loop_key(self.loop_flag);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct PreviewCmd {
    preview: Option<usize>,
}

impl PreviewCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_preview_key(self.preview);
        Ok(())
    }
}
