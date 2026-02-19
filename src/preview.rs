use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Show art preview
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "preview")]
pub struct CmdPreview {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// preview frame index
    #[argh(option, short = 'f', default = "0")]
    frame: usize,
}

impl CmdPreview {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }

        let frames = art.to_ansi_frames();

        if frames.len() > 0 {
            println!(
                "{}",
                frames[art
                    .get_preview_key()
                    .unwrap_or(self.frame)
                    .min(frames.len() - 1)]
            );
        } else {
            println!("There is no frames in art")
        };
        Ok(())
    }
}
