use anyhow::Result;
use argh::FromArgs;
use rs3a::{Cell, chars::Char};

/// Fill all frames or specific one with text and color
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "fill")]
pub struct CmdFill {
    /// text channel cell
    #[argh(option, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[argh(option, short = 'c')]
    color: Option<char>,

    /// frame index
    #[argh(option, short = 'f')]
    frame: Option<usize>,
}

impl CmdFill {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        if let Some(text) = self.text {
            if let Some(frame) = self.frame {
                art.fill_text_frame(frame, Char::new(text)?);
            } else {
                art.fill_text(Char::new(text)?);
            }
        }
        if let Some(color) = self.color {
            if let Some(frame) = self.frame {
                art.fill_color_frame(frame, Some(Char::new(color)?));
            } else {
                art.fill_color(Some(Char::new(color)?));
            }
        }
        Ok(())
    }
}

/// Fill area in all frames or specific one with text and color
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "fill-area")]
pub struct CmdFillArea {
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

    /// text channel cell
    #[argh(option, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[argh(option, short = 'c')]
    color: Option<char>,

    /// frame index
    #[argh(option, short = 'f')]
    frame: Option<usize>,
}

impl CmdFillArea {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        for row in self.rf..=self.rt {
            for col in self.cf..=self.ct {
                if let Some(frame) = self.frame {
                    let mut cell = art.get(frame, col, row, Cell::default());
                    if let Some(text) = self.text {
                        cell.text = Char::new(text)?;
                    }
                    if let Some(color) = self.color {
                        cell.color = Some(Char::new(color)?);
                    }
                    art.set(frame, col, row, cell);
                } else {
                    for frame in 0..art.frames() {
                        let mut cell = art.get(frame, col, row, Cell::default());
                        if let Some(text) = self.text {
                            cell.text = Char::new(text)?;
                        }
                        if let Some(color) = self.color {
                            cell.color = Some(Char::new(color)?);
                        }
                        art.set(frame, col, row, cell);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Fill all frames or specific one with default text, color
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "clean")]
pub struct CmdClean {
    /// frame index
    #[argh(option, short = 'f')]
    frame: Option<usize>,
}

impl CmdClean {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        if let Some(frame) = self.frame {
            art.clean_frame(frame);
        } else {
            art.clean();
        }
        Ok(())
    }
}
