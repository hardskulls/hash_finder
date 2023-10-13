#![allow(dead_code)]

use criterion::BenchmarkId;
use std::ops::{Add, Sub};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_finder::core::hashing::abstractions::gen_range::{
    AbstractNumber, GenRange, SingleNumAlias,
};
use hash_finder::core::hashing::types::numbers::Number;

fn range_generators(c: &mut Criterion) {
    let mut group = c.benchmark_group("[:: Function `range_generators` ::]");

    let start: Number = 1;
    let end: Number = 10_000;
    let f = |num| num;

    println!("[:: Benchmarking function `range_generators` ::]");
    println!("[!!] The range is '{}' [!!]", end.sub(start).add(1));

    for i in [1_u64, 2_u64, 3, 4, 5].iter() {
        group.bench_with_input(BenchmarkId::new("[AbstractNumber]", i), "", |b, _i| {
            b.iter(|| AbstractNumber::gen_range(black_box(start), black_box(end), black_box(f)))
        });
        group.bench_with_input(BenchmarkId::new("[SingleNumAlias]", i), "", |b, _i| {
            b.iter(|| SingleNumAlias::gen_range(black_box(start), black_box(end), black_box(f)))
        });
    }

    group.finish();
}

criterion_group!(benches, range_generators);
criterion_main!(benches);
