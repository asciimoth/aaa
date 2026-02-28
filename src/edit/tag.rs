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

/// Add tag to art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tag-add")]
pub struct CmdTagAdd {
    #[argh(positional)]
    tags: Vec<String>,
}

impl CmdTagAdd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        for tag in &self.tags {
            art.add_tag(tag);
        }
        Ok(())
    }
}

/// Remove tag from art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tag-rm")]
pub struct CmdTagRm {
    #[argh(positional)]
    tags: Vec<String>,
}

impl CmdTagRm {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        for tag in &self.tags {
            art.remove_tag(tag);
        }
        Ok(())
    }
}

/// Drop all tags
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tags-drop")]
pub struct CmdTagsDrop {}

impl CmdTagsDrop {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.remove_all_tags();
        Ok(())
    }
}
