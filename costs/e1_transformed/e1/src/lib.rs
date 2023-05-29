#[no_mangle]
unsafe fn unsafe_op(arr: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..arr.len() {
        res += *arr.get_unchecked(i);
    }
    res
}