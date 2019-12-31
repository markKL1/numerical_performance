
use std::process::exit;
use std::env::args;

//double* make_mat_empty(size_t n) {
//
//    double *mat = malloc(sizeof(double[n * n]));
//    if (mat == NULL) {
//        fprintf(stderr, "Failed to allocate\n");
//        fflush(stderr);
//    }
//
//    return mat;
//}
//
//double* make_mat_with_data(size_t n) {
//
//    double *mat = make_mat_empty(n);
//
//    for (size_t i = 0; i < n; i++) {
//        for (size_t j = 0; j < n; j++) {
//            double id = (double) i, jd = (double) j;
//            mat[i * n + j] = (id * id) / (jd + 1);
//        }
//    }
//
//    return mat;
//}
//
//double* mat_mul(size_t n, double A[], double B[]) {
//
//    double *C = make_mat_empty(n);
//
//    for (size_t i = 0; i < n; i++) {
//        for (size_t j = 0; j < n; j++) {
//            for (size_t k = 0; k < n; k++) {
//                 C[i * n + j] = A[i * n + k] * A[k * n + j];
//            }
//        }
//    }
//
//    return C;
//}
//
//double mat_elem_sum(size_t n, double mat[]) {
//
//    double sum = 0.0;
//
//    for (size_t i = 0; i < n; i++) {
//        for (size_t j = 0; j < n; j++) {
//            sum += mat[i * n + j];
//        }
//    }
//
//    return sum;
//}

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

//    // Make data.
//    double *A = make_mat_with_data(n);
//    double *B = make_mat_with_data(n);
//
//    // Array multiplication.
//    double *C = mat_mul(n, A, B);
//
//    // Determinant.
//    double det = mat_elem_sum(n, C);
//
//    fprintf(stderr, "sum of product elements = %.6f.\n", det);
//    fflush(stderr);
}
