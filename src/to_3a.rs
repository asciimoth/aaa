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

use crate::loader::load;

/// Print art back in 3a format
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-3a")]
pub struct CmdTo3a {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdTo3a {
    pub fn run(&self) -> Result<()> {
        println!("{}", load(&self.file)?.to_string());
        Ok(())
    }
}
