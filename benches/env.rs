use std::collections::{BTreeMap, HashMap};

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

use safenv as env;

fn benchmark_env(c: &mut Criterion) {
    c.bench_function("inherit env", |b| b.iter(|| unsafe { env::inherit() }));

    c.bench_function("inherit env black_box", |b| {
        b.iter(|| env::fill(black_box(std::env::vars_os())))
    });
}

fn benchmark_maps(c: &mut Criterion) {
    fn default_hashmap() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(black_box("FOO".to_string()), black_box("BAR".to_string()));
        map
    }

    fn default_btreemap() -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(black_box("FOO".to_string()), black_box("BAR".to_string()));
        map
    }

    c.bench_function("default hashmap", |b| b.iter(default_hashmap));
    c.bench_function("default btreemap", |b| b.iter(default_btreemap));

    c.bench_function("insert hashmap", |b| {
        b.iter_batched(
            default_hashmap,
            |mut map| map.insert(black_box("BAR".to_string()), black_box("FOO".to_string())),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("insert btreemap", |b| {
        b.iter_batched(
            default_btreemap,
            |mut map| map.insert(black_box("BAR".to_string()), black_box("FOO".to_string())),
            BatchSize::SmallInput,
        );
    });

    c.bench_function("remove hashmap", |b| {
        b.iter_batched(
            default_hashmap,
            |mut map| map.remove(black_box("FOO")),
            BatchSize::SmallInput,
        );
    });
    c.bench_function("remove btreemap", |b| {
        b.iter_batched(
            default_btreemap,
            |mut map| map.remove(black_box("FOO")),
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, benchmark_env, benchmark_maps);
criterion_main!(benches);
