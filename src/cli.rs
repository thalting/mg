use std::path::PathBuf;

use argh::FromArgs;

use crate::{ffi, Visitor};

#[derive(FromArgs)]
/// A simple and performant `GNU stow`-like dotfiles linker.
pub struct Options {
    /// the Git repository to read the dotfiles from
    #[argh(option, short = 's')]
    source: PathBuf,
    /// the directory the dotfiles will be linked to
    #[argh(option, default = "default_target()", short = 't')]
    target: PathBuf,
}

impl Options {
    pub fn into_visitor(self) -> Visitor {
        Visitor { source: self.source, target: self.target }
    }
}

fn default_target() -> PathBuf {
    println!("Source not supplied. Using home directory as target");

    ffi::home_directory()
}