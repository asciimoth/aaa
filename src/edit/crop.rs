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

/// Crop art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "crop")]
pub struct CmdCrop {
    /// from row
    #[argh(positional)]
    rf: usize,

    /// to row
    #[argh(positional)]
    rt: usize,

    /// from column
    #[argh(positional)]
    cf: usize,

    /// to columt
    #[argh(positional)]
    ct: usize,
}

impl CmdCrop {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.crop(self.rf, self.rt, self.cf, self.ct);
        Ok(())
    }
}
