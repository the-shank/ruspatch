/**
 * How to reproduce this bug:
 *     - This bug need to be reproduced in release build.
 *     - cargo run --release
 */
use std::alloc::{alloc, Layout};

pub struct FILE {
    buf: Vec<u8>,
}

pub unsafe fn _fdopen() {
    let layout = Layout::new::<FILE>();

    let f = alloc(layout) as *mut FILE;
    *f = FILE {
        buf: vec![0u8; 100],
    };
    // ptr::write(f, FILE{buf: vec![0u8; 100]});
}

fn main() {
    unsafe {
        _fdopen();
    }
}
