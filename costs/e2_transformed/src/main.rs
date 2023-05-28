use std::env;
use std::time::Instant;
fn main() {
    let args: Vec<String> = env::args().collect();
    test(args[1].parse::<usize>().unwrap());
}
fn test(ops_num: usize) {
    let arr = vec![1; 100];
    let arr_p = arr.as_ptr();
    let mut sum = 0;
    let start = Instant::now();
    unsafe {
        let lib_guard = e2::LIB.read().unwrap();
        let lib = lib_guard.as_ref().unwrap();
        let unsafe_op: libloading::Symbol<fn(arr: *const i32) -> i32> =
            lib.get(b"unsafe_op").unwrap();
        for _ in 0..ops_num {
            sum += unsafe_op(arr_p);
        }
    }
    let duration = start.elapsed();
    println!(
        "\"offset\" op num: {},  time spend: {:?}",
        ops_num, duration
    );
    println!("original sum :{}", sum);
}
unsafe fn unsafe_op(arr: *const i32) -> i32 {
    let mut res = 0;
    for i in 0..100 {
        res += *arr.offset(i);
    }
    res
}
