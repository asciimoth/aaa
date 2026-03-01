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
use rs3a::Art;

#[derive(clap::Args, PartialEq, Debug)]
pub struct StripCmd {
    /// art file path (alternatively pipe art to stdin)
    file: Option<String>,
}

impl StripCmd {
    pub fn run(&self, art: &mut Art) -> Result<()> {
        art.strip_comments();
        println!("{}", art.to_string());
        Ok(())
    }
}
