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
use std::io::{Write, stdout};

use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Convert art to ttyrec format
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-ttyrec")]
pub struct CmdToTtyrec {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,
}

impl CmdToTtyrec {
    pub fn run(&self) -> Result<()> {
        let mut out = stdout();
        out.write(&load(&self.file)?.to_ttyrec())?;
        Ok(())
    }
}
