use crate::core::hashing::types::numbers::NumberHash;

pub use ring_hasher::RingHasher;
mod ring_hasher;

#[cfg(feature = "sha256_hasher")]
pub use sha256_hasher::SHA256Hasher;
#[cfg(feature = "sha256_hasher")]
mod sha256_hasher;

#[cfg(feature = "openssl_hasher")]
pub use openssl_hasher::OpenSSLHasher;
#[cfg(feature = "openssl_hasher")]
mod openssl_hasher;

/// Returns a number and its hash if the number's hash ends with a specified number of zeros.
pub trait HashEndsWithNZeros<NUM, HASH> {
    fn matches(number: NUM, zeros: usize) -> Option<NumberHash<NUM, HASH>>;

    fn hash_this(bytes: &[u8]) -> HASH;
}

/// If there is enough zeroes at the end of a hash returns `true`.
fn enough_zeros_at_end(hash: &str, zeros: usize) -> bool {
    let mut idx = hash.len() - 1;
    let mut zeros_left = zeros;

    while zeros_left > 0 {
        if !matches!(hash.get(idx..=idx), Some("0")) {
            return false;
        }
        idx -= 1;
        zeros_left -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(any(feature = "openssl_hasher", feature = "sha256_hasher"))]
    use crate::core::hashing::abstractions::hasher::HashEndsWithNZeros;

    #[test]
    fn enough_zeros_test() {
        let hash = "jhbkj79876987";
        let zeros = 0;
        let res = enough_zeros_at_end(hash, zeros);
        assert!(res);

        let hash = "jhbkj79876987";
        let zeros = 1;
        let res = !enough_zeros_at_end(hash, zeros);
        assert!(res);

        let hash = "jhbkj79876980";
        let zeros = 1;
        let res = enough_zeros_at_end(hash, zeros);
        assert!(res);

        let hash = "jhbkj7987690";
        let zeros = 2;
        let res = !enough_zeros_at_end(hash, zeros);
        assert!(res);

        let hash = "jhbkj7987698700";
        let zeros = 2;
        let res = enough_zeros_at_end(hash, zeros);
        assert!(res);

        let hash = "jhbkj7987698700";
        let zeros = 3;
        let res = !enough_zeros_at_end(hash, zeros);
        assert!(res);

        let hash = "jhbkj79876987000";
        let zeros = 3;
        let res = enough_zeros_at_end(hash, zeros);
        assert!(res);
    }

    #[cfg(all(feature = "openssl_hasher", feature = "sha256_hasher"))]
    #[test]
    fn test_hashers_coherence() {
        let n = 8754890562_u64;

        let sha256 =
            <SHA256Hasher as HashEndsWithNZeros<u8, String>>::hash_this(n.to_string().as_bytes());
        let openssl =
            <OpenSSLHasher as HashEndsWithNZeros<u8, String>>::hash_this(n.to_string().as_bytes());
        let ring =
            <RingHasher as HashEndsWithNZeros<u8, String>>::hash_this(n.to_string().as_bytes());

        assert_eq!(sha256, openssl);
        assert_eq!(openssl, ring);
        assert_eq!(sha256, ring);
    }

    #[cfg(all(feature = "openssl_hasher", feature = "sha256_hasher"))]
    #[test]
    fn matches_comparison() {
        use crate::utils::MapType;

        // [!!] Same number will have completely different hash if it's type is changed.
        // [!!] This type should be u128 (for test purposes).
        let n: u128 = 483338;
        let reference = "50345144129e5b7e68d1d0e3cd2bdb48dcd55f1bad03a1a34ccd0296a0000000"
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
