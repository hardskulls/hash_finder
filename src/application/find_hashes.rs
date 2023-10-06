use crate::domain::hashing::abstractions::gen_range::{AbstractNumber, GenRange};
use crate::domain::hashing::abstractions::hasher::{HashEndsWithNZeros, RingHasher};
use crate::domain::hashing::value_objects::numbers::{Number, NumberHash};

/// Finds hashes containing enough zeros at the end and sends them through channel.
pub fn find_hashes<F>(start: Number, end: Number, with_zeros_at_end: usize, f: F)
where
    F: Fn(NumberHash<Number, String>) + Send + Sync,
{
    let f = |number| {
        if let Some(num_hash) = RingHasher::matches(number, with_zeros_at_end) {
            f(num_hash);
        }
    };
    AbstractNumber::gen_range(start, end, f);
}
