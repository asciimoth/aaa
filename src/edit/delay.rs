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

/// Set delay for whole art or for specific frame
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "delay-set")]
pub struct CmdDelaySet {
    /// delay in milliseconds
    #[argh(positional)]
    delay: usize,

    /// frame index
    #[argh(option, short = 'f')]
    frame: Option<usize>,
}

impl CmdDelaySet {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        if let Some(frame) = self.frame {
            art.set_frame_delay(frame, self.delay);
        } else {
            art.set_global_delay(self.delay);
        }
        Ok(())
    }
}

/// Reset all art delays to default (50 milis)
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "delay-reset")]
pub struct CmdDelayReSet {}

impl CmdDelayReSet {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.reset_delays(None);
        Ok(())
    }
}
