//use crate::is_valid;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use level_coin::pow::validator;

fn criterion_benchmark(c: &mut Criterion) {
    let bench = || {
        let result = validator::find(6, "yay");
        black_box(result);
    };

    c.bench_function(
        "yay",
        |b| b.iter(bench)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);