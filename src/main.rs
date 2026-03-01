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
mod cmd;
mod effects;
mod img;
mod loader;
mod player;

use crate::cmd::fetch::FetchCmd;
use crate::cmd::generate::GenCmd;
use crate::cmd::play::PlayCmd;
use crate::cmd::preview::PreviewCmd;
use crate::cmd::to::ConvertCmd;
use crate::cmd::{edit::EditCmd, from_text::FromTextCmd};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete_nushell::Nushell;
use cmd::list::ListCmd;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// List builtin art
    List(ListCmd),
    /// Generate new art
    Gen(GenCmd),
    /// Play art (or two side by side) in terminal
    Play(PlayCmd),
    /// Show system info side by side with animated logo. (by default requires one
    /// of fetch tools to be installed: neofetch | fastfetch | screenfetch | nitch | profetch | leaf |
    /// fetch-scm)
    Fetch(FetchCmd),
    /// Show art preview
    Preview(PreviewCmd),
    /// Editing subcommands
    Edit(EditCmd),
    /// Format conversion subcommands
    Convert(ConvertCmd),
    /// Constructs art from plain text with ANSI color escape codes
    FromText(FromTextCmd),

    /// Generate shell completions to stdout (shell: bash|zsh|fish|powershell|elvish)
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
    Nush,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List(cmd) => cmd.run(),
        Commands::Gen(cmd) => cmd.run(),
        Commands::Play(cmd) => cmd.run(),
        Commands::Fetch(cmd) => cmd.run(),
        Commands::Preview(cmd) => cmd.run(),
        Commands::Edit(cmd) => cmd.run(),
        Commands::Convert(cmd) => cmd.run(),
        Commands::FromText(cmd) => cmd.run(),
        Commands::Completions { shell } => {
            generate_completions(shell);
            Ok(())
        }
    }
}

fn generate_completions(shell: Shell) {
    use clap_complete::{generate, shells};

    // Build the clap Command from our root Parser
    let mut cmd = Cli::command();

    match shell {
        Shell::Bash => generate(shells::Bash, &mut cmd, "aaa", &mut std::io::stdout()),
        Shell::Zsh => generate(shells::Zsh, &mut cmd, "aaa", &mut std::io::stdout()),
        Shell::Fish => generate(shells::Fish, &mut cmd, "aaa", &mut std::io::stdout()),
        Shell::PowerShell => generate(shells::PowerShell, &mut cmd, "aaa", &mut std::io::stdout()),
        Shell::Elvish => generate(shells::Elvish, &mut cmd, "aaa", &mut std::io::stdout()),
        Shell::Nush => generate(Nushell, &mut cmd, "aaa", &mut std::io::stdout()),
    }
}
