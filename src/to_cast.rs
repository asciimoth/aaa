use anyhow::Result;
use argh::FromArgs;

use crate::loader::load;

/// Convert art to asciicast v2 format
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "to-cast")]
pub struct CmdToCast {
    /// art file path (alternatively pipe art to stdin)
    #[argh(positional)]
    file: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,
}

impl CmdToCast {
    pub fn run(&self) -> Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }

        println!("{}", art.to_asciicast2());
        Ok(())
    }
}
