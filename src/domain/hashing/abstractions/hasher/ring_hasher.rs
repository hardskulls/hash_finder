use crate::domain::hashing::abstractions::hasher::{enough_zeros_at_end, HashEndsWithNZeros};
use crate::domain::hashing::value_objects::numbers::NumberHash;
use crate::utils::MapType;
use num_traits::{Num, ToBytes};

// [!!] Fully-qualified path have to be used to avoid error `E0282`.
/// Implements `HashEndsWithNZeros` using `ring` crate.
pub struct RingHasher;

impl<N> HashEndsWithNZeros<N, String> for RingHasher
where
    N: Num + ToBytes,
{
    fn matches(number: N, zeros: usize) -> Option<NumberHash<N, String>> {
        <Self as HashEndsWithNZeros<N, _>>::hash_this(number.to_be_bytes().as_ref())
            .map_type(Some)
            // .map(|n| { dbg!(&n); n })
            .filter(|h| enough_zeros_at_end(h, zeros))
            .map(|hash| NumberHash { number, hash })
    }

    fn hash_this(bytes: &[u8]) -> String {
        let hash = ring::digest::digest(&ring::digest::SHA256, bytes);
        let res = hash.as_ref();

        hex::encode(res)
    }
}
