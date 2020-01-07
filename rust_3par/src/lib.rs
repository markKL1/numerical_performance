#![allow(non_snake_case)]

use ::faster::*;
use ::std::time::Instant;
use ::par_array_init::par_array_init;
use ::rayon::iter::IntoParallelIterator;
use ::rayon::iter::ParallelIterator;
use ::rayon::iter::IntoParallelRefIterator;
use ::rayon::slice::ParallelSliceMut;

const STEP: usize = 8;

fn make_mat_empty(n: usize) -> Vec<f64> {
    let mut v: Vec<f64> = Vec::with_capacity(n * n);
    unsafe { v.set_len(n * n); }
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

    let mut BT: Vec<Vec<f64>> = (0 .. n).into_par_iter().map(|j| {
        let mut Bc = make_mat_empty(n);
        for m in 0..n {
            unsafe {
                Bc[m] = *B.get_unchecked(m * n + j);
            }
        }
        Bc
    }).collect::<Vec<_>>();

    //TODO @mverleg: https://docs.rs/rayon/1.3.0/rayon/slice/trait.ParallelSliceMut.html#tymethod.as_parallel_slice_mut

    let mut C = make_mat_empty(n);
    //TODO @mverleg: should this be called row or column?
    C.par_chunks_mut(n)
        .for_each(|row| {
            let j = 0;  //TODO @mverleg: TEMPORARY! REMOVE THIS!
            dbg!(row.len());
            let Bc = &BT[j];

            for i in 0 .. n {
                let ni = i * n;
                let mut k = 0;
                let stop_simd_at = n - (n % STEP);
                while k < stop_simd_at {
                    debug_assert!(8 == STEP);
                    let nik = ni + k;
                    unsafe {
                        row[j] += A.get_unchecked(nik + 0) * Bc.get_unchecked(k + 0);
                        row[j] += A.get_unchecked(nik + 1) * Bc.get_unchecked(k + 1);
                        row[j] += A.get_unchecked(nik + 2) * Bc.get_unchecked(k + 2);
                        row[j] += A.get_unchecked(nik + 3) * Bc.get_unchecked(k + 3);
                        row[j] += A.get_unchecked(nik + 4) * Bc.get_unchecked(k + 4);
                        row[j] += A.get_unchecked(nik + 5) * Bc.get_unchecked(k + 5);
                        row[j] += A.get_unchecked(nik + 6) * Bc.get_unchecked(k + 6);
                        row[j] += A.get_unchecked(nik + 7) * Bc.get_unchecked(k + 7);
                    }
                    k += STEP;
                }
                for k in stop_simd_at .. n {
                    unsafe {
                        row[j] += A.get_unchecked(ni + k) * Bc.get_unchecked(k);
                    }
                }
            }

        });

    //TODO @mverleg: sequential_threshold(n)
//    (0 .. n).into_par_iter()
//        .map(|i| C[i .. i + n])
//        .for_each(|s| s.len());

//
//    C.as_mut_slice().into_par_iter().batched();
//    C.as_mut_slice().into_par_iter()
//        .batch
//        .zip((0 .. Array::len()).into_par_iter().map(|i| initializer(i)))
//        //TODO @mverleg: batched?
//        .for_each(|dst, src| *dst = src);

//    let iter = into_iter.into_par_iter();
//    if Array::len() > iter.len() {
//        return None;
//    }
//    let mut ret: Array = unsafe { std::mem::uninitialized() };
//    ret.mut_slice()
//        .into_par_iter()
//        .zip(iter)
//        .for_each(|(dst, src)| {
//            *dst = src;
//        });
//    Some(ret);
//
//    let C: Vec<f64> = par_array_init(|m| {
//        dbg!(m);
//    });

//    par_array_init
//
//    (0 .. n).into_par_iter().map(|j| {
//        let mut Bc = vec![0.0; n];
//        for m in 0 .. n {
//            unsafe {
//                Bc[m] = *B.get_unchecked(m * n + j);
//            }
//        }
//        let Bc = Bc;  // immutable
//        for i in 0..n {
//            let ni = i * n;
//            C[ni + j] = (
//                Bc.simd_iter(f64s(0.0)),
//                A[ni .. (ni + n)].simd_iter(f64s(0.0)),
//            ).zip()
//                .simd_map(|(a, b)| a * b)
//                .simd_reduce(f64s(0.0), |acc, v| acc + v)
//                .sum();
//        }
//    });

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