#![allow(non_snake_case)]

mod jni_wrap;

use ::faster::*;
use ::rayon::iter::IndexedParallelIterator;
use ::rayon::iter::IntoParallelIterator;
use ::rayon::iter::ParallelIterator;
use ::rayon::slice::ParallelSliceMut;
use ::std::time::Instant;

fn make_mat_empty(n: usize) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::with_capacity(n * n);
    unsafe { v.set_len(n * n); }
    v
}

fn make_vec_empty(n: usize) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::with_capacity(n);
    unsafe { v.set_len(n); }
    v
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

    // Save a sort-of-transposed version of B, for fast indexing.
    let BT: Vec<Vec<f64>> = (0 .. n).into_par_iter().map(|j| {
        let mut Bc = make_vec_empty(n);
        for m in 0 .. n {
            unsafe {
                *Bc.get_unchecked_mut(m) = *B.get_unchecked(m * n + j);
            }
        }
        Bc
    }).collect::<Vec<_>>();

    // Do the multiplication C = A * B
    // Outer loop uses threads, and inner loop uses simd (middle loop is normal).
    // see https://docs.rs/rayon/1.3.0/rayon/slice/trait.ParallelSliceMut.html#tymethod.as_parallel_slice_mut
    let mut C = make_mat_empty(n);
    C.par_chunks_mut(n)
        .enumerate()
        .for_each(|(j, c)| {
            let Bc = &BT[j];
            for i in 0 .. n {
                let ni = i * n;
                c[i] = (
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
    let sum = mat_elem_sum(n, C) / ((n * n) as f64);

    let time_s = timer.elapsed().as_nanos() as f64 / 1.0e9;

    (sum, time_s)
}