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
