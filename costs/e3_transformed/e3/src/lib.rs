use std::ptr::copy_nonoverlapping;
#[no_mangle]
unsafe fn unsafe_op(src: *const i32, dst: *mut i32, count: usize) {
    copy_nonoverlapping(src, dst, count);
}

