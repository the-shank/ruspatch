#[no_mangle]
unsafe fn unsafe_op(arr: *const i32) -> i32 {
    let mut res = 0;
    for i in 0..100 {
        res += *arr.offset(i);
    }
    res
}