use std::env;
use std::ptr::copy_nonoverlapping;
use std::time::Instant;
fn main() {
    let args: Vec<String> = env::args().collect();
    let bench_num = args[2].parse::<usize>().unwrap();
    for _ in 0..bench_num{
        test(args[1].parse::<usize>().unwrap());
    }
}
fn test(ops_num: usize) {
    let src = vec![1; 100];
    let src_ptr = src.as_ptr();
    let mut dst = vec![2; 100];
    let dst_ptr = dst.as_mut_ptr();
    let start = Instant::now();
    unsafe {
        let lib_guard = e3::LIB.read().unwrap();
        let lib = lib_guard.as_ref().unwrap();
        let unsafe_op: libloading::Symbol<fn(src: *const i32, dst: *mut i32, count: usize)> =
            lib.get(b"unsafe_op").unwrap();
        for _ in 0..ops_num {
            unsafe_op(src_ptr, dst_ptr, 100);
        }
    }
    let duration = start.elapsed();
    println!("{} ", duration.as_nanos());
    // println!(
    //     "\"get_unchecked\" op num: {},  time spend: {:?}",
    //     ops_num, duration
    // );
}
unsafe fn unsafe_op(src: *const i32, dst: *mut i32, count: usize) {
    copy_nonoverlapping(src, dst, count);
}
