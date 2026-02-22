use anyhow::Result;
use argh::FromArgs;

/// Set art title
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "title")]
pub struct CmdTitle {
    #[argh(positional)]
    title: Option<String>,
}

impl CmdTitle {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_title_key(self.title.clone());
        Ok(())
    }
}

/// Set authors
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "authors")]
pub struct CmdAuthors {
    #[argh(positional)]
    authors: Vec<String>,
}

impl CmdAuthors {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_authors_key(&self.authors);
        Ok(())
    }
}

/// Set orig authors
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "origs")]
pub struct CmdOrigs {
    #[argh(positional)]
    authors: Vec<String>,
}

impl CmdOrigs {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_orig_authors_key(&self.authors);
        Ok(())
    }
}

/// Set src
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "src")]
pub struct CmdSrc {
    #[argh(positional)]
    src: Option<String>,
}

impl CmdSrc {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_src_key(self.src.clone());
        Ok(())
    }
}

/// Set ditor
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "editor")]
pub struct CmdEditor {
    #[argh(positional)]
    editor: Option<String>,
}

impl CmdEditor {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_editor_key(self.editor.clone());
        Ok(())
    }
}

/// Set license
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "license")]
pub struct CmdLicense {
    #[argh(positional)]
    license: Option<String>,
}

impl CmdLicense {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_license_key(self.license.clone());
        Ok(())
    }
}

/// Set loop
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "loop")]
pub struct CmdLoop {
    #[argh(positional, long = "loop")]
    loop_flag: bool,
}

impl CmdLoop {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_loop_key(self.loop_flag);
        Ok(())
    }
}

/// Set preview
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "preview")]
pub struct CmdPreview {
    #[argh(positional)]
    preview: Option<usize>,
}

impl CmdPreview {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.set_preview_key(self.preview);
        Ok(())
    }
}
