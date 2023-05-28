use std::env;
use std::time::Instant;
fn main() {
    let args: Vec<String> = env::args().collect();
    let bench_num = args[2].parse::<usize>().unwrap();
    for _ in 0..bench_num{
        test(args[1].parse::<usize>().unwrap());
    }
    
}
fn test(ops_num: usize) {
    let arr = vec![1; 100];
    let start = Instant::now();
    let mut sum = 0;
    unsafe {
        let lib_guard = e1::LIB.read().unwrap();
        let lib = lib_guard.as_ref().unwrap();
        let unsafe_op: libloading::Symbol<fn(arr: &[i32]) -> i32> = lib.get(b"unsafe_op").unwrap();
        for _ in 0..ops_num {
            sum += unsafe_op(&arr);
        }
    }
    let duration = start.elapsed();
    println!(
        "\"get_unchecked\" op num: {},  time spend: {:?}",
        ops_num, duration
    );
    println!("original sum :{}", sum);
}
unsafe fn unsafe_op(arr: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..arr.len() {
        res += *arr.get_unchecked(i);
    }
    res
}
