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
use rs3a::{Cell, chars::Char};

#[derive(clap::Args, PartialEq, Debug)]
pub struct SetCmd {
    /// frame number
    frame: usize,

    /// row number
    row: usize,

    /// column number
    column: usize,

    /// text channel cell
    #[clap(long, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[clap(long, short = 'c')]
    color: Option<char>,
}

impl SetCmd {
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
