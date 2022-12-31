use std::{ffi::OsStr, os::unix::fs::symlink, path::Path};

use ffi::ToPath;
use ignore::{DirEntry, WalkBuilder};

mod ffi;

pub type StaticPath = &'static Path;

pub struct Visitor {
    pub(crate) home_directory: &'static Path,
    pub(crate) dotfiles_repository: &'static Path,
}

impl Visitor {
    pub fn new(dotfiles_repository: &'static OsStr) -> Self {
        Self {
            home_directory: ffi::home_directory().to_path(),
            dotfiles_repository: Path::new(dotfiles_repository),
        }
    }

    pub fn symlink_files(&self) {
        let dir_walker = WalkBuilder::new(self.dotfiles_repository)
            .follow_links(false)
            // TODO: maybe this should be true
            .require_git(false)
            .build();

        // How many components in the path of the dotfiles repository
        let dotfiles_repo_components =
            self.dotfiles_repository.components().count();

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

            if is_git_file(&entry) {
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
                path.strip_prefix(self.dotfiles_repository).unwrap();

            let target = self.home_directory.join(sub_path);

            if let Err(err) = symlink(path, &target) {
                eprintln!(
                    "Failed to symlink {path:?} to {target:?}: {err}"
                );
            }

            println!("Linked {path:?} to {target:?}");
        }
    }
}

fn main() {
    let dotfiles_repo = argv::iter()
        .nth(1)
        .expect("Dotfiles repository not supplied");

    let visitor = Visitor::new(dotfiles_repo);
    visitor.symlink_files();
}
