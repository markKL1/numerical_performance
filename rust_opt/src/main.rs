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
        for m in 0 .. n {
            //TODO @mverleg: unchecked?
            Bc[m] = B[m * n + j];
        }
        for i in 0 .. n {
            let ni = i * n;
            // Deal with the remainder modulo $STEP.
            for k in (n - n % STEP) .. n {
                //TODO @mverleg: unchecked?
                C[i * n + j] += A[i * n + k] * Bc[k];
            }
            // Do the rest in steps of $STEP.
            let mut sums = [0.0; STEP];
            let mut k = n - (n % STEP);
            loop {
                k -= STEP;
                debug_assert!(STEP == 8);
                if ni + k + STEP - 1 > A.len() || k + STEP - 1 > Bc.len() {
                    use ::std::hint::unreachable_unchecked;
                    //TODO @mverleg: why doesn't this import?
                    unsafe { unreachable_unchecked!(); }
                }
                sums[7] += A[ni + k + 7] * Bc[k + 7];
                sums[6] += A[ni + k + 6] * Bc[k + 6];
                sums[5] += A[ni + k + 5] * Bc[k + 5];
                sums[4] += A[ni + k + 4] * Bc[k + 4];
                sums[3] += A[ni + k + 3] * Bc[k + 3];
                sums[2] += A[ni + k + 2] * Bc[k + 2];
                sums[1] += A[ni + k + 1] * Bc[k + 1];
                sums[0] += A[ni + k + 0] * Bc[k + 0];
                if k <= 0 {
                    break;
                }
            }
            C[ni + j] += sums.iter().sum::<f64>();
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
