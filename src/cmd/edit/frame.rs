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
pub struct FrameRemoveCmd {
    /// frame index
    frame: usize,
}

impl FrameRemoveCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.remove_frame(self.frame);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameDupCmd {
    /// frame index
    frame: usize,
}

impl FrameDupCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.dup_frame(self.frame);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameSureCmd {
    /// frame index
    frame: usize,
}

impl FrameSureCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.make_sure_frame_exist(self.frame);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameSliceCmd {
    /// range start
    from: usize,

    /// range end
    to: usize,
}

impl FrameSliceCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.slice(self.from, self.to);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameSwapCmd {
    /// first frame
    a: usize,

    /// second frame
    b: usize,
}

impl FrameSwapCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.swap(self.a, self.b);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameRevCmd {}

impl FrameRevCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.reverse();
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameDedupCmd {}

impl FrameDedupCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.dedup();
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameRotForthCmd {
    k: usize,
}

impl FrameRotForthCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.rot_forth(self.k);
        Ok(())
    }
}

#[derive(clap::Args, PartialEq, Debug)]
pub struct FrameRotBackCmd {
    k: usize,
}

impl FrameRotBackCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.rot_back(self.k);
        Ok(())
    }
}
