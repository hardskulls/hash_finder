use crate::hashing::funcs::enough_zeros_at_end;
use crate::hashing::traits::HashEndsWithNZeros;
use crate::MapType;

/// Stores a number and its sha256 hash.
#[derive(Debug, Clone)]
pub struct NumberHash<I, H> {
    pub number: I,
    pub hash: H,
}

pub(super) type Number = u128;

/// Implements `HashEndsWithNZeros` using `sha256` crate.
pub struct SHA256Hasher;

impl HashEndsWithNZeros<Number, String> for SHA256Hasher {
    fn matches(number: Number, zeros: usize) -> Option<NumberHash<Number, String>> {
        Self::hash_this(&number.to_ne_bytes())
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }

    fn hash_this(bytes: &[u8]) -> String {
        sha256::digest(bytes)
    }
}

/// Implements `HashEndsWithNZeros` using `openssl` crate.
pub struct OpenSSLHasher;

impl HashEndsWithNZeros<Number, String> for OpenSSLHasher {
    fn matches(number: Number, zeros: usize) -> Option<NumberHash<Number, String>> {
        Self::hash_this(&number.to_ne_bytes())
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }

    fn hash_this(bytes: &[u8]) -> String {
        let mut hasher = openssl::sha::Sha256::new();
        hasher.update(bytes);
        let res = hasher.finish();

        hex::encode(res)
    }
}

/// Implements `HashEndsWithNZeros` using `ring` crate.
pub struct RingHasher;

impl HashEndsWithNZeros<Number, String> for RingHasher {
    fn matches(number: Number, zeros: usize) -> Option<NumberHash<Number, String>> {
        Self::hash_this(&number.to_ne_bytes())
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }

    fn hash_this(bytes: &[u8]) -> String {
        let hash = ring::digest::digest(&ring::digest::SHA256, bytes);
        let res = hash.as_ref();

        hex::encode(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashers_coherence() {
        let n = 8754890562_u64;

        let sha256 = SHA256Hasher::hash_this(n.to_string().as_bytes());
        let openssl = OpenSSLHasher::hash_this(n.to_string().as_bytes());
        let ring = RingHasher::hash_this(n.to_string().as_bytes());

        assert_eq!(sha256, openssl);
        assert_eq!(openssl, ring);
        assert_eq!(sha256, ring);
    }

    #[test]
    fn matches_comparison() {
        let n = 647562409;
        let reference = "6fe17e0a64c61512b7b1b1d80813d3f2b141b8d9aa11450b75d6867a00000000"
            .to_string()
            .map_type(Some);

        let sha256 = SHA256Hasher::matches(n, 3).map(|n| n.hash);
        let openssl = OpenSSLHasher::matches(n, 3).map(|n| n.hash);
        let ring = RingHasher::matches(n, 3).map(|n| n.hash);

        assert_eq!(reference, sha256);
        assert_eq!(reference, openssl);
        assert_eq!(reference, ring);
    }
}

/// Testing module used for micro-benchmarking.
#[cfg(test)]
mod benches {
    use super::*;

    fn hash_num(n: &[u8]) -> String {
        let mut hash = openssl::sha::Sha256::new();
        hash.update(n);
        let res = hash.finish();

        hex::encode(res)
    }

    #[test]
    fn bench_sha256_vs_openssl_hashers() {
        let times = 1_000_000;
        let data = 647562409;
        let zeros = 7;

        let execute = || SHA256Hasher::matches(data, zeros);
        let sha256_version = pocket_micro_benching::bench_times(times, execute).unwrap();

        let execute = || OpenSSLHasher::matches(data, zeros);
        let openssl_version = pocket_micro_benching::bench_times(times, execute).unwrap();

        assert_eq!(sha256_version, openssl_version);
    }

    #[test]
    fn bench_openssl_vs_ring_hashers() {
        let times = 1_000_000;
        let data = 647562409;
        let zeros = 7;

        let execute = || RingHasher::matches(data, zeros);
        let ring_version = pocket_micro_benching::bench_times(times, execute).unwrap();

        let execute = || OpenSSLHasher::matches(data, zeros);
        let openssl_version = pocket_micro_benching::bench_times(times, execute).unwrap();

        assert_eq!(ring_version, openssl_version);
    }

    #[test]
    fn bench_num_vs_string_hashing() {
        let times = 1_000_000;
        let n = 7621340988765_u64;

        let exec = || hash_num(&n.to_ne_bytes());
        let native_conversion = pocket_micro_benching::bench_times(times, exec).unwrap();

        let exec = || hash_num(n.to_string().as_bytes());
        let string_conversion = pocket_micro_benching::bench_times(times, exec).unwrap();

        assert_eq!(native_conversion, string_conversion);
    }
}
