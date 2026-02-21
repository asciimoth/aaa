use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Print art as a blank line separated sequence of frames
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-frames")]
pub struct CmdToFrames {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,
}

impl CmdToFrames {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }

        for frame in art.to_ansi_frames() {
            println!("{}\n", frame)
        }
        Ok(())
    }
}
