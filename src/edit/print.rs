use anyhow::Result;
use argh::FromArgs;
use rs3a::chars::Char;

/// Print text to art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "print")]
pub struct CmdPrint {
    /// frame number
    #[argh(positional)]
    frame: usize,

    /// row number
    #[argh(positional)]
    row: usize,

    /// column number
    #[argh(positional)]
    column: usize,

    #[argh(positional)]
    text: String,

    /// text color
    #[argh(option, short = 'c')]
    color: Option<char>,
}

impl CmdPrint {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        let color = if let Some(color) = self.color {
            Some(Some(Char::new(color)?))
        } else {
            None
        };
        art.print(self.frame, self.column, self.row, &self.text, color);
        Ok(())
    }
}

/// Print text with ansi color codes to art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "print-ansi")]
pub struct CmdPrintANSI {
    /// frame number
    #[argh(positional)]
    frame: usize,

    /// row number
    #[argh(positional)]
    row: usize,

    /// column number
    #[argh(positional)]
    column: usize,

    #[argh(positional)]
    text: String,
}

impl CmdPrintANSI {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.print_ansi(self.frame, self.column, self.row, &self.text);
        Ok(())
    }
}
