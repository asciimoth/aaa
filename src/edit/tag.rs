use anyhow::Result;
use argh::FromArgs;

/// Add tag to art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tag-add")]
pub struct CmdTagAdd {
    #[argh(positional)]
    tags: Vec<String>,
}

impl CmdTagAdd {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        for tag in &self.tags {
            art.add_tag(tag);
        }
        Ok(())
    }
}

/// Remove tag from art
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tag-rm")]
pub struct CmdTagRm {
    #[argh(positional)]
    tags: Vec<String>,
}

impl CmdTagRm {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        for tag in &self.tags {
            art.remove_tag(tag);
        }
        Ok(())
    }
}

/// Drop all tags
#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "tags-drop")]
pub struct CmdTagsDrop {}

impl CmdTagsDrop {
    pub fn run(&self, art: &mut rs3a::Art) -> Result<()> {
        art.remove_all_tags();
        Ok(())
    }
}
