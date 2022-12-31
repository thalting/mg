use std::{ffi::CStr, ops::Not};

pub fn getenv(key: &CStr) -> Option<&'static CStr> {
    unsafe {
        let res = libc::getenv(key.as_ptr());

        res.is_null().not().then(|| CStr::from_ptr(res))
    }
}
