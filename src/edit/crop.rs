use std::str::FromStr;

use anyhow::Result;
use argh::FromArgs;
use rs3a::{art, chars::Char, Color, ColorPair};

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
