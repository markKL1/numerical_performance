#![allow(non_snake_case)]

use ::std::env::args;
use ::std::process::exit;
use ::std::time::Instant;

const STEP: usize = 8;

fn make_mat_empty(n: usize) -> Vec<f64> {
    vec![0.0; n * n]
}

fn make_mat_with_data(n: usize, s: f64) -> Vec<f64> {

    let mut mat = make_mat_empty(n);

    for i in 0 .. n {
        for j in 0 .. n {
            let id = i as f64;
            let jd = j as f64;
            mat[i * n + j] = (id * id) / (jd + 1.0 + s);
        }
    }

    return mat;
}

fn mat_mul(n: usize, A: Vec<f64>, B: Vec<f64>) -> Vec<f64> {

    let mut C = make_mat_empty(n);

    let mut Bc = vec![0.0; n];
    for j in 0 .. n {
        for k in 0 .. n {
            unsafe {
                Bc[k] = *B.get_unchecked(k * n + j);
            }
        }
        for i in 0..n {
            let ni = i * n;
            let mut k = 0;
            let stop_simd_at = n - (n % STEP);
            while k < stop_simd_at {
                debug_assert!(8 == STEP);
                unsafe {
                    C[ni + j] += A.get_unchecked(ni + k + 0) * Bc.get_unchecked(k + 0);
                    C[ni + j] += A.get_unchecked(ni + k + 1) * Bc.get_unchecked(k + 1);
                    C[ni + j] += A.get_unchecked(ni + k + 2) * Bc.get_unchecked(k + 2);
                    C[ni + j] += A.get_unchecked(ni + k + 3) * Bc.get_unchecked(k + 3);
                    C[ni + j] += A.get_unchecked(ni + k + 4) * Bc.get_unchecked(k + 4);
                    C[ni + j] += A.get_unchecked(ni + k + 5) * Bc.get_unchecked(k + 5);
                    C[ni + j] += A.get_unchecked(ni + k + 6) * Bc.get_unchecked(k + 6);
                    C[ni + j] += A.get_unchecked(ni + k + 7) * Bc.get_unchecked(k + 7);
                }
                k += STEP;
            }
            for k in stop_simd_at .. n {
                unsafe {
                    C[ni + j] += A.get_unchecked(ni + k) * Bc.get_unchecked(k);
                }
            }
        }
    }

    return C;
}

fn mat_elem_sum(n: usize, mat: Vec<f64>) -> f64 {

    let mut sum = 0.0;

    for i in 0 .. n {
        for j in 0 .. n {
            sum += mat[i * n + j];
        }
    }

    return sum;
}

fn main() {
    eprintln!("Hello world from Rust!");

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

    // Make data.
    let A: Vec<f64> = make_mat_with_data(n, 1.0);
    let B: Vec<f64> = make_mat_with_data(n, 3.0);

    let timer = Instant::now();

    // Array multiplication.
    let C: Vec<f64> = mat_mul(n, A, B);

    // Sum elements.
    let sum = mat_elem_sum(n, C) / (n as f64);

    let cpu_time_used = timer.elapsed().as_nanos() as f64 / 1.0e9;

    eprintln!("Sum of product elements = {:.6}.", sum);
    eprintln!("Time taken = {:.6} second.", cpu_time_used);
}
