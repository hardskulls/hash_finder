#![allow(dead_code)]

use criterion::BenchmarkId;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hash_finder::domain::hashing::abstractions::hasher::HashEndsWithNZeros;
#[cfg(feature = "openssl_hasher")]
use hash_finder::domain::hashing::abstractions::hasher::OpenSSLHasher;
use hash_finder::domain::hashing::abstractions::hasher::RingHasher;
#[cfg(feature = "sha256_hasher")]
use hash_finder::domain::hashing::abstractions::hasher::SHA256Hasher;

fn bench_hashers(c: &mut Criterion) {
    let mut group = c.benchmark_group("[:: SHA-256 Hashing Implementations ::]");
    let number = 645423907764433577642235_u128.to_string();
    let n = black_box(number.as_bytes());
    for i in [1_u64, 2_u64, 3, 4, 5].iter() {
        #[cfg(feature = "sha256_hasher")]
        group.bench_with_input(BenchmarkId::new("[sha256_hasher]", i), n, |b, i| {
            b.iter(|| <SHA256Hasher as HashEndsWithNZeros<u8, String>>::hash_this(i))
        });
        #[cfg(feature = "openssl_hasher")]
        group.bench_with_input(BenchmarkId::new("[openssl_hasher]", i), n, |b, i| {
            b.iter(|| <OpenSSLHasher as HashEndsWithNZeros<u8, String>>::hash_this(i))
        });
        group.bench_with_input(BenchmarkId::new("[ring_hasher]", i), n, |b, i| {
            b.iter(|| <RingHasher as HashEndsWithNZeros<u8, String>>::hash_this(i))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_hashers);
criterion_main!(benches);
