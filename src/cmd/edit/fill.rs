/*
    This file is part of aaa.

    aaa is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    aaa is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with aaa.  If not, see <https://www.gnu.org/licenses/>.
*/
use anyhow::Result;
use rs3a::{Cell, chars::Char};

#[derive(clap::Args, PartialEq, Debug)]
pub struct FillCmd {
    /// text channel cell
    #[clap(long, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[clap(long, short = 'c')]
    color: Option<char>,

    /// frame index
    #[clap(long, short = 'f')]
    frame: Option<usize>,
}

impl FillCmd {
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

#[derive(clap::Args, PartialEq, Debug)]
pub struct FillAreaCmd {
    /// from row
    rf: usize,

    /// to row
    rt: usize,

    /// from column
    cf: usize,

    /// to columt
    ct: usize,

    /// text channel cell
    #[clap(long, short = 't')]
    text: Option<char>,

    /// color channel cell
    #[clap(long, short = 'c')]
    color: Option<char>,

    /// frame index
    #[clap(long, short = 'f')]
    frame: Option<usize>,
}

impl FillAreaCmd {
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

#[derive(clap::Args, PartialEq, Debug)]
pub struct CleanCmd {
    /// frame index
    #[clap(long, short = 'f')]
    frame: Option<usize>,
}

impl CleanCmd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        if let Some(frame) = self.frame {
            art.clean_frame(frame);
        } else {
            art.clean();
        }
        Ok(())
    }
}
