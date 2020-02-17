#![allow(non_snake_case)]

use ::std::env::args;
use ::std::process::exit;
use ::num_perf_demo::mul_test;


fn main() {
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        eprintln!("Provide integer argument.");
        exit(1);
    }
    let n: usize = args[1].parse::<usize>().unwrap();
    if n <= 1 {
        eprintln!("Dimension argument too small.");
        exit(1);
    }

    let (sum, cpu_time_used) = mul_test(n);

    eprintln!("Sum of product elements = {:.6}.", sum);
    eprintln!("Time taken = {:.6} second.", cpu_time_used);
}
