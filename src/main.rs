use std::{os::unix::fs::symlink, path::{Path, PathBuf}};

use cli::Options;
use ignore::{DirEntry, WalkBuilder};

mod cli;
mod ffi;

pub type StaticPath = &'static Path;

pub struct Visitor {
    /// The Git repository we will read the dotfiles from
    pub(crate) source: PathBuf,
    /// The target directory where the dotfiles will be symlinked to
    pub(crate) target: PathBuf,
}

impl Visitor {
    pub fn new(dotfiles_repository: PathBuf) -> Self {
        Self {
            target: ffi::home_directory(),
            source: dotfiles_repository,
        }
    }

    pub fn symlink_files(&self) {
        let dir_walker = WalkBuilder::new(&self.source)
            .follow_links(false)
            // TODO: maybe this should be true
            .require_git(false)
            .build();

        // How many components in the path of the dotfiles repository
        let dotfiles_repo_components =
            self.source.components().count();

        // Returns true if this entry is part of the Git repository
        // that holds the supplied dotfiles
        let is_git_file = |entry: &DirEntry| -> bool {
            let path = entry.path();

            path.components()
                .nth(dotfiles_repo_components)
                .map(|component| component.as_os_str() == ".git")
                .unwrap_or(false)
        };

        for entry in dir_walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    // tomou kkkkkkk
                    eprintln!("Error while dirwalking: {err}");
                    continue;
                }
            };

            dbg!(entry.path());

            if entry.path().is_dir()
                || entry.path() == Path::new(".")
                || is_git_file(&entry)
            {
                continue;
            }

            let path = entry.path();
            // Safety: should not fail since we're walking through
            // `self.dotfiles_repository` therefore all
            // paths start with `self.dotfiles_repository`. The
            // exception to this rule would be if we were
            // somewhere else due to following a symlink, but WalkDir
            // is set to not follow symlinks
            let sub_path =
                path.strip_prefix(&self.source).unwrap();

            let target = self.target.join(sub_path);

            if let Err(err) = symlink(path, &target) {
                eprintln!(
                    "Failed to symlink {path:?} to {target:?}: {err}"
                );
                continue;
            }

            println!("Linked {path:?} to {target:?}");
        }
    }
}

fn main() {
    let options: Options = argh::from_env();
    
    options.into_visitor().symlink_files()
}
