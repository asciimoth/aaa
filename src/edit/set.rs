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

/// Set cell (text, color)
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "set")]
pub struct CmdSet {
    /// frame number
    #[argh(positional)]
    frame: usize,

    /// row number
    #[argh(positional)]
    row: usize,

    /// column number
    #[argh(positional)]
    column: usize,

    /// text channel cell
    #[argh(option, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[argh(option, short = 'c')]
    color: Option<char>,
}

impl CmdSet {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        let mut cell = art.get(self.frame, self.column, self.row, Cell::default());
        if let Some(text) = self.text {
            cell.text = Char::new(text)?;
        }
        if let Some(color) = self.color {
            cell.color = Some(Char::new(color)?);
        }
        art.set(self.frame, self.column, self.row, cell);
        Ok(())
    }
}
