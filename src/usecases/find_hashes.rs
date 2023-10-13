use crate::core::hashing::abstractions::gen_range::{GenRange, SingleNumAlias};
use crate::core::hashing::abstractions::hasher::{HashEndsWithNZeros, RingHasher};
use crate::core::hashing::types::numbers::{Number, NumberHash};

/// Finds hashes containing enough zeros at the end and applies `f` to the result.
pub fn find_hashes<F>(start: Number, end: Number, with_zeros_at_end: usize, f: F)
where
    F: Fn(NumberHash<Number, String>) + Send + Sync,
{
    let f = |number| {
        if let Some(num_hash) = RingHasher::matches(number, with_zeros_at_end) {
            f(num_hash);
        }
    };
    SingleNumAlias::gen_range(start, end, f);
}
