#![allow(dead_code)]

use criterion::BenchmarkId;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn hash_num(bytes: &[u8]) -> String {
    let hash = ring::digest::digest(&ring::digest::SHA256, bytes);
    let res = hash.as_ref();

    hex::encode(res)
}

fn bench_bench_num_vs_string_hashing(c: &mut Criterion) {
    let mut group = c.benchmark_group("[:: Hashing number VS Hashing Number as String ::]");

    let n = black_box(7621340988765_u128);

    let native_conversion = || hash_num(&n.to_ne_bytes());
    let string_conversion = || hash_num(n.to_string().as_bytes());

    for i in [1_u64, 2_u64].iter() {
        group.bench_with_input(
            BenchmarkId::new("[native number hashing]", i),
            "",
            |b, _i| b.iter(native_conversion),
        );
        group.bench_with_input(
            BenchmarkId::new("[hashing number converted to string]", i),
            "",
            |b, _i| b.iter(string_conversion),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_bench_num_vs_string_hashing);
criterion_main!(benches);
