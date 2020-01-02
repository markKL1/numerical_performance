
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <time.h>

double* make_mat_empty(size_t n) {

    double *mat = malloc(sizeof(double[n * n]));
    if (mat == NULL) {
        fprintf(stderr, "Failed to allocate\n");
        fflush(stderr);
    }

    return mat;
}

double* make_mat_with_data(size_t n, double s) {

    double *mat = make_mat_empty(n);

    for (size_t i = 0; i < n; i++) {
        for (size_t j = 0; j < n; j++) {
            double id = (double) i, jd = (double) j;
            mat[i * n + j] = (id * id) / (jd + 1 + s);
        }
    }

    return mat;
}

double* mat_mul(size_t n, double A[], double B[]) {
    // https://rosettacode.org/wiki/Matrix_multiplication#Java

    double *C = make_mat_empty(n);

    for (size_t i = 0; i < n; i++) {
        for (size_t j = 0; j < n; j++) {
            for (size_t k = 0; k < n; k++) {
                 C[i * n + j] += A[i * n + k] * B[k * n + j];
            }
        }
    }

    return C;
}

double mat_elem_sum(size_t n, double mat[]) {

    double sum = 0.0;

    for (size_t i = 0; i < n; i++) {
        for (size_t j = 0; j < n; j++) {
            sum += mat[i * n + j];
        }
    }

    return sum;
}

//double* mat_determinant(size_t n, double mat[]) {
//
//    double det = 0.0;
//    int[] xmask =
//
//
//    return det;
//}
//
//double* submat_determinant(size_t n, size_t subn, int[] xmask, int[] ymask, double mat[]) {
//
//
//
//    double det = 0.0;
//
//    for (size_t i = 0; i < n; i++) {
//        for (size_t j = 0; j < n; j++) {
//            for (size_t k = 0; k < n; k++) {
//
//            }
//        }
//    }
//
//    return det;
//}

int main(int argc, char *argv[]) {

    fprintf(stderr, "Hello world from C!\n");
    fflush(stderr);

    if (argc <= 1) {
        printf("Provide integer argument.\n");
        fflush(stderr);
        exit(1);
    }
    size_t n = (size_t)atoi(argv[1]);
    if (n <= 1) {
        printf("Dimension argument too small.\n");
        fflush(stderr);
        exit(1);
    }

    // Make data.
    double *A = make_mat_with_data(n, 1.0);
    double *B = make_mat_with_data(n, 3.0);

    clock_t start = clock();

    // Array multiplication.
    double *C = mat_mul(n, A, B);

    // Sum elements.
    double sum = mat_elem_sum(n, C) / ((double)n);

    clock_t end = clock();
    double cpu_time_used = ((double) (end - start)) / CLOCKS_PER_SEC;

    fprintf(stderr, "Sum of product elements = %.6f.\n", sum);
    fprintf(stderr, "Time taken = %.6f second.\n", cpu_time_used);
    fflush(stderr);
}

