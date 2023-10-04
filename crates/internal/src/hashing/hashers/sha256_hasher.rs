use crate::hashing::funcs::enough_zeros_at_end;
use crate::hashing::types::NumberHash;
use crate::hashing::HashEndsWithNZeros;
use crate::utils::MapType;

/// Implements `HashEndsWithNZeros` using `sha256` crate.
pub struct SHA256Hasher;

impl<N> HashEndsWithNZeros<N, String> for SHA256Hasher
where
    N: num_traits::Num + num_traits::ToBytes,
{
    fn matches(number: N, zeros: usize) -> Option<NumberHash<N, String>> {
        <Self as HashEndsWithNZeros<N, String>>::hash_this(number.to_ne_bytes().as_ref())
            .map_type(Some)
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }

    fn hash_this(bytes: &[u8]) -> String {
        sha256::digest(bytes)
    }
}
