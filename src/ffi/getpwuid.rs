use std::{ffi::CStr, mem, ptr};

use libc::{c_char, getpwuid_r, passwd};

static mut BUF: [c_char; 2048] = [0; 2048];

fn effective_user_id() -> u32 {
    unsafe { libc::geteuid() }
}

pub fn home_dir() -> Option<&'static CStr> {
    // Safety: it is safe to modify BUF since there are no other
    // threads accessing it
    let buf = unsafe { &mut BUF };

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

        return Some(home_dir);
    }

    None
}
