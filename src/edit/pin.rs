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

/// Pin text channel
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pin-text")]
pub struct CmdPinText {
    /// frame for pin
    #[argh(positional)]
    frame: usize,
}

impl CmdPinText {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.pin_text(self.frame)?;
        Ok(())
    }
}

/// Pin color channel
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pin-color")]
pub struct CmdPinColor {
    /// frame for pin
    #[argh(positional)]
    frame: usize,
}

impl CmdPinColor {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.pin_color(self.frame)?;
        Ok(())
    }
}
