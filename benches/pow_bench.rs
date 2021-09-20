//use crate::is_valid;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use level_coin::pow::validator;
use rand::{distributions::Alphanumeric, Rng};

fn criterion_benchmark(c: &mut Criterion) {
    let prefix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();

    let bench = || {
        let stat = validator::find(4, prefix.as_str());
        //println!("{:?}", stat);
        black_box(stat);
    };

    c.bench_function(
        "yay",
        |b| b.iter(bench)
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);