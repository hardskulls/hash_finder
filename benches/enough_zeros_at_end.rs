#![allow(dead_code)]

use criterion::BenchmarkId;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Copy-pasted here from `internal/hashing/funcs` to keep it private.
/// Not a big deal, but it was intended to be internal.
///
/// If there is enough zeroes at the end of a hash returns `true`.
pub fn enough_zeros_at_end(hash: &str, zeros: usize) -> bool {
    let mut idx = hash.len() - 1;
    let mut zeros_left = zeros;

    while zeros_left > 0 {
        if !matches!(hash.get(idx..idx + 1), Some("0")) {
            return false;
        }
        idx -= 1;
        zeros_left -= 1;
    }

    true
}

fn bench_enough_zeros_at_end(c: &mut Criterion) {
    let mut group = c.benchmark_group("[:: Function `enough_zeros_at_end` ::]");

    let s = black_box("64543527890000000");
    let zeros = black_box(7);

    for i in [1_u64, 2_u64].iter() {
        group.bench_with_input(
            BenchmarkId::new("[passing params 1st method]", i),
            "",
            |b, _i| b.iter(|| enough_zeros_at_end(s, zeros)),
        );
        group.bench_with_input(
            BenchmarkId::new("[passing params 2nd method]", i),
            &(s, zeros),
            |b, i| b.iter(|| enough_zeros_at_end(i.0, i.1)),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_enough_zeros_at_end);
criterion_main!(benches);
