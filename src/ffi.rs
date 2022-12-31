use std::path::PathBuf;

mod getpwuid;

pub fn home_directory() -> PathBuf {
    match std::env::var_os("HOME") {
        Some(home) => home.into(),
        None => {
            // Fallback to reading the passwd entry for the euid of
            // this process
            getpwuid::home_dir()
                .expect("Failed to obtain home directory")
        }
    }
}
