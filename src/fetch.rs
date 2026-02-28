use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

use anyhow::{Result, anyhow};
use argh::FromArgs;
use rs3a::Art;

use crate::{
    edit::effects::Effect,
    loader::{BuiltIn, load},
    player::play,
};

/// Show system info side by side with animated logo. (by default requires one
/// of fetch tools to be installed: neofetch | fastfetch | screenfetch | nitch | profetch | leaf |
/// fetch-scm)
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "fetch")]
pub struct CmdFetch {
    /// art name or file path (alternatively pipe art to stdin and set `--art --`)
    #[argh(option, short = 'a')]
    art: Option<String>,

    /// disable colors
    #[argh(switch, short = 'n')]
    no_colors: bool,

    /// whether loop aniamtion
    #[argh(option, long = "loop", short = 'l')]
    loop_flag: Option<bool>,

    /// horisontal art offset
    #[argh(option, short = 'o', default = "0")]
    art_offset: usize,

    /// info block horisontal offset
    #[argh(option, default = "0")]
    info_offset: usize,

    /// animation effect: roller_up | roller_down | roller_left | roller_right
    #[argh(option, default = "Effect::RollerUp")]
    effect: Effect,

    /// fetch command to run
    #[argh(positional)]
    cmd: Vec<String>,
}

impl CmdFetch {
    fn get_art(&self) -> Result<Art> {
        if let Some(art) = self.art.clone() {
            if art == "--" {
                load(&None)
            } else {
                load(&Some(art))
            }
        } else {
            let os = env::consts::OS;
            load(&Some(if os == "linux" {
                let mut logo_name = String::from("generic_linux");
                let distro = distro_info();
                for art in BuiltIn::iter() {
                    if let Some(art) = art.strip_prefix("distro_") {
                        let art = art.strip_suffix(".3a").unwrap();
                        if distro.contains(art) {
                            logo_name = art.into();
                        }
                    }
                }
                format!("distro_{}", logo_name)
            } else {
                "question".into()
            }))
        }
    }
    pub fn run(&self) -> Result<()> {
        let text = if self.cmd.len() > 0 {
            run_cmd(&self.cmd)?
        } else {
            run_fetches()?
        };
        let info = {
            let mut info = Art::from_ansi_text(&text);
            self.effect.apply(&mut info, 0);
            info.set_loop_key(false);
            if self.no_colors {
                info.set_colors_key(Some(false));
            }
            info
        };
        let art = {
            let mut art = self.get_art()?;
            if self.no_colors {
                art.set_colors_key(Some(false));
            }
            if let Some(flag) = self.loop_flag {
                art.set_loop_key(flag);
            }
            art
        };
        play(&art, Some(&info), self.art_offset, self.info_offset + 1)?;
        Ok(())
    }
}

fn run_cmd(cmd: &[String]) -> Result<String> {
    let mut command = Command::new(&cmd[0]);
    command.args(&cmd[1..]);
    Ok(String::from_utf8(command.output()?.stdout)?)
}

fn run_fetches() -> Result<String> {
    let fetches: [Vec<String>; _] = [
        vec!["neofetch".into(), "--off".into()],
        "fastfetch --logo none --pipe false"
            .split(" ")
            .map(|s| s.into())
            .collect(),
        vec!["screenfetch".into(), "-n".into()],
        vec!["nitch".into()],
        vec!["profetch".into()],
        vec!["leaf".into()],
        vec!["fetch-scm".into()],
    ];
    for fetch in fetches {
        if let Ok(out) = run_cmd(&fetch) {
            return Ok(out);
        }
    }
    Err(anyhow!("no supported fetches found"))
}

fn distro_info() -> String {
    let mut info = String::new();
    info += &load_os_release();
    info += " ";
    info += &load_lsb_release();
    info.to_lowercase()
}

fn load_os_release() -> String {
    let mut info = String::new();
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some((key, value)) = line.split_once('=') {
                if key == "NAME" || key == "PRETTY_NAME" || key == "ID" {
                    info += value;
                    info += " ";
                }
            }
        }
    }
    info
}

fn load_lsb_release() -> String {
    let mut info = String::new();
    if let Ok(file) = File::open("/etc/lsb-release") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if let Some((key, value)) = line.split_once('=') {
                if key == "DISTRIB_ID" {
                    info += value;
                    info += " ";
                }
            }
        }
    }
    info
}
