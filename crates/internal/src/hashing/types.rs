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
pub struct StringSHA256Hasher;

impl HashEndsWithNZeros<u128, String> for StringSHA256Hasher {
    fn matches(number: u128, zeros: usize) -> Option<NumberHash<u128, String>> {
        sha256::digest(number.to_string())
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }
}

/// Implements `HashEndsWithNZeros` using `openssl` crate.
pub struct OpenSSLHasher;

impl HashEndsWithNZeros<u128, String> for OpenSSLHasher {
    fn matches(number: u128, zeros: usize) -> Option<NumberHash<u128, String>> {
        let mut hash = openssl::sha::Sha256::new();
        hash.update(number.to_string().as_bytes());
        let res = hash.finish();

        hex::encode(res)
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_hashers() {
        let times = 1_000_000;
        let data = 85070591730234615865843651857942510189_u128;
        let zeros = 7;

        let execute = || StringSHA256Hasher::matches(data, zeros);
        let sha256_version = pocket_micro_benching::bench_times(times, execute).unwrap();
        
        let execute = || OpenSSLHasher::matches(data, zeros);
        let openssl_version = pocket_micro_benching::bench_times(times, execute).unwrap();
        
        assert_eq!(sha256_version, openssl_version);
    }
}
