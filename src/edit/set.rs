use anyhow::Result;
use argh::FromArgs;
use rs3a::{chars::Char, Cell};

/// Set cell (text, color)
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "set")]
pub struct CmdSet {
    /// frame number
    #[argh(positional)]
    frame: usize,

    /// row number
    #[argh(positional)]
    row: usize,

    /// column number
    #[argh(positional)]
    column: usize,

    /// text channel cell
    #[argh(option, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[argh(option, short = 'c')]
    color: Option<char>,
}

impl CmdSet {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        let mut cell = art.get(self.frame, self.column, self.row, Cell::default());
        if let Some(text) = self.text {
            cell.text = Char::new(text)?;
        }
        if let Some(color) = self.color {
            cell.color = Some(Char::new(color)?);
        }
        art.set(self.frame, self.column, self.row, cell);
        Ok(())
    }
}
