use std::{
    ffi::{CStr, OsString},
    mem,
    os::unix::prelude::OsStringExt,
    path::PathBuf,
    ptr,
};

use libc::{c_char, getpwuid_r, passwd};

fn effective_user_id() -> u32 {
    unsafe { libc::geteuid() }
}

pub fn home_dir() -> Option<PathBuf> {
    let mut buf: [c_char; 2048] = [0; 2048];

    let mut result = ptr::null_mut();
    let mut passwd: passwd = unsafe { mem::zeroed() };

    let uid = effective_user_id();

    let getpwuid_r_code = unsafe {
        getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr(),
            buf.len(),
            &mut result,
        )
    };

    if getpwuid_r_code == 0 && !result.is_null() {
        // Safety: getpwuid_r worked so `passwd.pw_dir` points to a
        // valid C string within static memory (i.e. within BUF)
        let home_dir = unsafe { CStr::from_ptr(passwd.pw_dir) };
        let bytes = home_dir.to_bytes().to_vec();

        return Some(OsString::from_vec(bytes).into());
    }

    None
}
