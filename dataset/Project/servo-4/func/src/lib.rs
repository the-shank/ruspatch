extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;
#[no_mangle]
unsafe fn c_str_to_string(s: *const c_char) -> String {
    str::from_utf8(CStr::from_ptr(s).to_bytes()).unwrap().to_owned()
}
