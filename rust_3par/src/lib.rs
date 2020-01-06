#![allow(non_snake_case)]

use ::faster::*;
use ::std::time::Instant;
use ::rayon::prelude::*;


fn make_mat_empty(n: usize) -> Vec<f64> {
    vec![0.0; n * n]
}

pub fn make_mat_with_data(n: usize, s: f64) -> Vec<f64> {

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

pub fn mat_mul(n: usize, A: Vec<f64>, B: Vec<f64>) -> Vec<f64> {

    let mut C = make_mat_empty(n);

    (0 .. n).into_par_iter().map(|j| {
        let mut Bc = vec![0.0; n];
        for m in 0 .. n {
            unsafe {
                Bc[m] = *B.get_unchecked(m * n + j);
            }
        }
        let Bc = Bc;  // immutable
        for i in 0..n {
            let ni = i * n;
            C[ni + j] = (
                Bc.simd_iter(f64s(0.0)),
                A[ni .. (ni + n)].simd_iter(f64s(0.0)),
            ).zip()
                .simd_map(|(a, b)| a * b)
                .simd_reduce(f64s(0.0), |acc, v| acc + v)
                .sum();
        }
    });

    return C;
}

pub fn mat_elem_sum(n: usize, mat: Vec<f64>) -> f64 {

    let mut sum = 0.0;

    for i in 0 .. n {
        sum += &mat[i * n .. i * n + n].simd_iter(f64s(0.0))
            .simd_reduce(f64s(0.0), |acc, v| acc + v)
            .sum();
    }

    return sum;
}

pub fn mul_test(n: usize) -> (f64, f64) {

    // Make data.
    let A: Vec<f64> = make_mat_with_data(n, 1.0);
    let B: Vec<f64> = make_mat_with_data(n, 3.0);

    let timer = Instant::now();

    // Array multiplication.
    let C: Vec<f64> = mat_mul(n, A, B);

    // Sum elements.
    let sum = mat_elem_sum(n, C) / (n as f64);

    let time_s = timer.elapsed().as_nanos() as f64 / 1.0e9;

    (sum, time_s)
}