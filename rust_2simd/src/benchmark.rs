use std::time::Duration;

use ::criterion::Criterion;
use ::criterion::criterion_group;
use ::criterion::criterion_main;
use criterion::{Benchmark, black_box};

use matmullib::mul_test;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "matmul",
        Benchmark::new("test", |b| b.iter(|| mul_test(black_box(800))))
            .sample_size(10)
            .measurement_time(Duration::from_millis(2000))
            .measurement_time(Duration::from_millis(18000))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

