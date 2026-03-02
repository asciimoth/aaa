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
pub mod cast;
pub mod gif;
pub mod mp4;
pub mod png;
pub mod svg;
pub mod webp;
use std::io::{Write, stdout};

use crate::{
    cmd::to::{cast::ToCastCmd, gif::GifCmd, mp4::Mp4Cmd, png::PngCmd, svg::SvgCmd, webp::WebpCmd},
    loader::load,
};
use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct ConvertCmd {
    /// art file path (alternatively pipe art to stdin)
    file: Option<String>,

    #[command(subcommand)]
    command: ConvertSubcommands,

    /// disable colors
    #[arg(short = 'n', long)]
    no_colors: bool,
}

#[derive(Debug, Subcommand)]
enum ConvertSubcommands {
    /// Print art as a blank line separated sequence of frames with ANSI colors codes.
    ToFrames,
    /// Convert art to asciicast v2 format
    ToCast(ToCastCmd),
    /// Convert art to durformat (durdraw's ascii art format)
    ToDur,
    /// Convert art to json document
    ToJson,
    /// Convert art to ttyrec format
    ToTtyrec,
    /// Convert art to png image
    ToPng(PngCmd),
    /// Convert art to gif animation
    ToGif(GifCmd),
    /// Convert art to webp animation (ffmpeg cli required)
    ToWebp(WebpCmd),
    /// Convert art to mp4 video (ffmpeg cli required)
    ToMp4(Mp4Cmd),
    /// Convert art to svg animation
    ToSvg(SvgCmd),
    /// Print art back in 3a format
    To3a,
}

impl ConvertCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut art = load(&self.file)?;
        if self.no_colors {
            art.set_colors_key(Some(false));
        }

        match &self.command {
            ConvertSubcommands::ToPng(cmd) => cmd.run(&mut art),
            ConvertSubcommands::ToGif(cmd) => cmd.run(&mut art),
            ConvertSubcommands::ToWebp(cmd) => cmd.run(&mut art),
            ConvertSubcommands::ToMp4(cmd) => cmd.run(&mut art),
            ConvertSubcommands::ToSvg(cmd) => cmd.run(&mut art),
            ConvertSubcommands::ToCast(cmd) => cmd.run(&mut art),
            ConvertSubcommands::To3a => {
                println!("{}", art.to_string());
                Ok(())
            }
            ConvertSubcommands::ToFrames => {
                for frame in art.to_ansi_frames() {
                    println!("{}\n", frame)
                }
                Ok(())
            }
            ConvertSubcommands::ToDur => {
                println!("{}", art.to_dur());
                Ok(())
            }
            ConvertSubcommands::ToJson => {
                println!("{}", art.to_json());
                Ok(())
            }
            ConvertSubcommands::ToTtyrec => {
                let mut out = stdout();
                out.write(&art.to_ttyrec())?;
                Ok(())
            }
        }
    }
}
