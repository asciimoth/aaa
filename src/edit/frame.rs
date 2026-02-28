use anyhow::Result;
use argh::FromArgs;

/// Remove frame
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-rm")]
pub struct CmdFrameRemove {
    /// frame index
    #[argh(positional)]
    frame: usize,
}

impl CmdFrameRemove {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.remove_frame(self.frame);
        Ok(())
    }
}

/// Duplicate frame
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-dup")]
pub struct CmdFrameDup {
    /// frame index
    #[argh(positional)]
    frame: usize,
}

impl CmdFrameDup {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.dup_frame(self.frame);
        Ok(())
    }
}

/// Ensures a frame exists at the given index,
/// creating new frames if necessary
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-ensure")]
pub struct CmdFrameSure {
    /// frame index
    #[argh(positional)]
    frame: usize,
}

impl CmdFrameSure {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.make_sure_frame_exist(self.frame);
        Ok(())
    }
}

/// Remove all frames out of inclusive subrange
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-slice")]
pub struct CmdFrameSlice {
    /// range start
    #[argh(positional)]
    from: usize,

    /// range end
    #[argh(positional)]
    to: usize,
}

impl CmdFrameSlice {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.slice(self.from, self.to);
        Ok(())
    }
}

/// Swap two frames
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-swap")]
pub struct CmdFrameSwap {
    /// first frame
    #[argh(positional)]
    a: usize,

    /// second frame
    #[argh(positional)]
    b: usize,
}

impl CmdFrameSwap {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.swap(self.a, self.b);
        Ok(())
    }
}

/// Reverse frames
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-rev")]
pub struct CmdFrameRev {}

impl CmdFrameRev {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.reverse();
        Ok(())
    }
}

/// Deduplicate frames
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-dedup")]
pub struct CmdFrameDedup {}

impl CmdFrameDedup {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.dedup();
        Ok(())
    }
}

/// Rotate frames forth
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-r-f")]
pub struct CmdFrameRotForth {
    #[argh(positional)]
    k: usize,
}

impl CmdFrameRotForth {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.rot_forth(self.k);
        Ok(())
    }
}

/// Rotate frames back
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "frame-r-b")]
pub struct CmdFrameRotBack {
    #[argh(positional)]
    k: usize,
}

impl CmdFrameRotBack {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.rot_back(self.k);
        Ok(())
    }
}
