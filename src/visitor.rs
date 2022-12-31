use std::{
    io,
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use ignore::{DirEntry, Walk, WalkBuilder};
use nix::errno::Errno;

pub struct Visitor {
    /// The Git repository we will read the dotfiles from
    pub(crate) source: PathBuf,
    /// The target directory where the dotfiles will be symlinked to
    pub(crate) target: PathBuf,
}

impl Visitor {
    fn build_walker(&self) -> Walk {
        WalkBuilder::new(&self.source)
            .follow_links(false)
            // TODO: maybe this should be true
            .require_git(false)
            .max_depth(Some(1))
            // Most dotfiles are hidden so it wouldn't make sense to
            // ignore them
            .hidden(false)
            .build()
    }

    fn do_operation(
        &self,
        path: &Path,
        target: &Path,
    ) -> io::Result<()> {
        if let Err(err) = symlink(path, target) {
            let res = match Errno::last() {
                Errno::ENOENT => {
                    // symlink failed because some component in target
                    // did not exist. Let's try to
                    // build it and then retry.
                    let parent = target.parent().unwrap();
                    std::fs::create_dir_all(parent)?;

                    symlink(path, target)
                }
                Errno::EEXIST => {
                    println!("Link at {target:?} already exists");
                    Ok(())
                }
                _ => Err(err),
            };

            return res;
        }

        Ok(())
    }

    fn build_target_link_path(&self, to_be_linked: &Path) -> PathBuf {
        // Safety: should not fail since we're walking through
        // `self.source` therefore all paths start with `self.source`.
        // The exception to this rule would be if we were
        // somewhere else due to following a symlink, but WalkDir
        // is set to not follow symlinks
        let sub_path =
            to_be_linked.strip_prefix(&self.source).unwrap();

        self.target.join(sub_path)
    }

    pub fn symlink_files(&self) {
        let dir_walker = self.build_walker();

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

        let is_source = |entry: &DirEntry| -> bool {
            let path = entry.path();

            path == Path::new(".") || path == self.source
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

            if is_source(&entry) || is_git_file(&entry) {
                continue;
            }

            let path = entry.path();
            let target = self.build_target_link_path(path);

            match self.do_operation(path, &target) {
                Ok(()) => println!("Linked {path:?} to {target:?}"),
                Err(err) => {
                    eprintln!("Failed to link {path:?} to {target:?}: {err}");
                }
            }
        }
    }
}
