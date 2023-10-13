use crate::core::hashing::abstractions::hasher::{enough_zeros_at_end, HashEndsWithNZeros};
use crate::core::hashing::types::numbers::NumberHash;
use crate::utils::MapType;
use num_traits::{Num, ToBytes};

// [!!] Fully-qualified path have to be used to avoid error `E0282`.
/// Implements `HashEndsWithNZeros` using `sha256` crate.
pub struct SHA256Hasher;

impl<N> HashEndsWithNZeros<N, String> for SHA256Hasher
where
    N: Num + ToBytes,
{
    fn matches(number: N, zeros: usize) -> Option<NumberHash<N, String>> {
        <Self as HashEndsWithNZeros<N, _>>::hash_this(number.to_be_bytes().as_ref())
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }

    fn hash_this(bytes: &[u8]) -> String {
        sha256::digest(bytes)
    }
}
