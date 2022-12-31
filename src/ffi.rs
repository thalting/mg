use std::{
    ffi::{CStr, OsStr},
    os::unix::prelude::OsStrExt,
    path::Path,
};

mod getenv;
mod getpwuid;

use cstr::cstr;
use getenv::getenv;

pub fn home_directory() -> &'static CStr {
    match getenv(cstr!("HOME")) {
        Some(home) => home,
        None => {
            // Fallback to reading the passwd entry for the euid of
            // this process
            getpwuid::home_dir()
                .expect("Failed to obtain home directory")
        }
    }
}

pub trait ToOsStr {
    fn to_os_str(&self) -> &'static OsStr;
}

impl ToOsStr for &'static CStr {
    fn to_os_str(&self) -> &'static OsStr {
        OsStr::from_bytes(self.to_bytes())
    }
}

pub trait ToPath {
    fn to_path(&self) -> &'static Path;
}

impl ToPath for &'static CStr {
    fn to_path(&self) -> &'static Path {
        Path::new(self.to_os_str())
    }
}
