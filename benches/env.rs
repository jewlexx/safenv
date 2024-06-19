use criterion::{black_box, criterion_group, criterion_main, Criterion};

use safenv as env;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("inherit env", |b| b.iter(|| unsafe { env::inherit() }));

    c.bench_function("inherit env black_box", |b| {
        b.iter(|| env::fill(black_box(std::env::vars_os())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
