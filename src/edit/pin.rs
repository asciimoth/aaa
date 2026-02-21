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
