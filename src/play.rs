use anyhow::Result;
use argh::FromArgs;

use crate::{loader::load, player::play};

/// Play art (or two side by side) in terminal
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "play")]
pub struct CmdPlay {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// secondary art file path
    #[argh(option)]
    secondary: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// whether loop aniamtion
    #[argh(option, long = "loop")]
    loop_flag: Option<bool>,

    /// horisontal offset
    #[argh(option, short = 'o', default = "0")]
    offset: usize,

    /// secondary art horisontal offset
    #[argh(option, default = "0")]
    secondary_offset: usize,
}

impl CmdPlay {
    pub fn run(&self) -> Result<()> {
        let primary = {
            let mut art = load(&self.file)?;
            if self.no_colors {
                art.set_colors_key(Some(false));
            }
            if let Some(flag) = self.loop_flag {
                art.set_loop_key(flag);
            }
            art
        };
        let secondary = match &self.secondary {
            Some(file) => {
                let mut art = load(&Some(file.clone()))?;
                if self.no_colors {
                    art.set_colors_key(Some(false));
                }
                if let Some(flag) = self.loop_flag {
                    art.set_loop_key(flag);
                }
                Some(art)
            }
            None => None,
        };

        play(
            &primary,
            secondary.as_ref(),
            self.offset,
            self.secondary_offset,
        )?;
        Ok(())
    }
}
