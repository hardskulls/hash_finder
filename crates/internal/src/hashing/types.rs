use crate::hashing::funcs::enough_zeros_at_end;
use crate::hashing::traits::HashEndsWithNZeros;
use crate::MapType;

/// Stores a number and its sha256 hash.
#[derive(Debug, Clone)]
pub struct NumberHash<I, H> {
    pub number: I,
    pub hash: H,
}

/// Implements `HashEndsWithNZeros` using `sha256` crate.
pub struct SHA256Hasher;

impl HashEndsWithNZeros<u128, String> for SHA256Hasher {
    fn matches(number: u128, zeros: usize) -> Option<NumberHash<u128, String>> {
        Self::hash_this(number.to_string().as_bytes())
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

impl HashEndsWithNZeros<u128, String> for OpenSSLHasher {
    fn matches(number: u128, zeros: usize) -> Option<NumberHash<u128, String>> {
        Self::hash_this(number.to_string().as_bytes())
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

impl HashEndsWithNZeros<u128, String> for RingHasher {
    fn matches(number: u128, zeros: usize) -> Option<NumberHash<u128, String>> {
        Self::hash_this(number.to_string().as_bytes())
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

    fn hash_num(n: &[u8]) -> String {
        let mut hash = openssl::sha::Sha256::new();
        hash.update(n);
        let res = hash.finish();

        hex::encode(res)
    }

    #[test]
    fn bench_sha256_vs_openssl_hashers() {
        let times = 1_000_000;
        let data = 85070591730234615865843651857942510189_u128;
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
        let data = 85070591730234615865843651857942510189_u128;
        let zeros = 7;

        let execute = || RingHasher::matches(data, zeros);
        let ring_version = pocket_micro_benching::bench_times(times, execute).unwrap();

        let execute = || OpenSSLHasher::matches(data, zeros);
        let openssl_version = pocket_micro_benching::bench_times(times, execute).unwrap();

        assert_eq!(ring_version, openssl_version);
    }

    #[test]
    fn test_hashers_coherence() {
        let n = 8754890562_u128;

        let sha256 = SHA256Hasher::hash_this(n.to_string().as_bytes());
        let openssl = OpenSSLHasher::hash_this(n.to_string().as_bytes());
        let ring = RingHasher::hash_this(n.to_string().as_bytes());

        assert_eq!(sha256, openssl);
        assert_eq!(openssl, ring);
        assert_eq!(sha256, ring);
    }

    #[test]
    fn number_as_bytes() {
        let number = 4567329_u128;
        let reference = hash_num(number.to_string().as_bytes());

        let num_as_str = hash_num(number.to_string().as_bytes());
        // let num_as_be_bytes = hash_num(&number.to_be_bytes());
        let num_as_le_bytes = hash_num(&number.to_le_bytes());
        let num_as_ne_bytes = hash_num(&number.to_ne_bytes());

        dbg!(number.to_string().as_bytes());
        dbg!(number.to_be_bytes());
        dbg!(number.to_le_bytes());
        dbg!(number.to_ne_bytes());

        assert_eq!(reference, num_as_str);
        // assert_eq!(num_as_le_bytes, num_as_be_bytes);
        assert_eq!(num_as_le_bytes, num_as_ne_bytes);
        // assert_eq!(reference, num_as_ne_bytes);
    }

    #[test]
    fn bench_num_vs_string_hashing() {
        let times = 1_000_000;
        let n = 7621340988765_u128;

        let exec = || hash_num(&n.to_ne_bytes());
        let native_conversion = pocket_micro_benching::bench_times(times, exec).unwrap();

        let exec = || hash_num(n.to_string().as_bytes());
        let string_conversion = pocket_micro_benching::bench_times(times, exec).unwrap();

        assert_eq!(native_conversion, string_conversion);
    }

    #[test]
    fn big_vs_little_endianness() {
        let times = 1_000_000;
        let n = 7621340988765_u128;

        let exec = || hash_num(&n.to_be_bytes());
        let big_endian = pocket_micro_benching::bench_times(times, exec).unwrap();

        let exec = || hash_num(&n.to_le_bytes());
        let little_endian = pocket_micro_benching::bench_times(times, exec).unwrap();

        assert_eq!(big_endian, little_endian);
    }
}
