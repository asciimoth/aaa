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
pub struct CropCmd {
    /// from row
    rf: usize,

    /// to row
    rt: usize,

    /// from column
    cf: usize,

    /// to columt
    ct: usize,
}

impl CropCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.crop(self.rf, self.rt, self.cf, self.ct);
        Ok(())
    }
}
