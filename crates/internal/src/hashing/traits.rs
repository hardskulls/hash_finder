use crate::hashing::types::NumberHash;

/// Returns `Option` of `NumberHash` if number's hash ends with a specified number of zeros.
pub trait HashEndsWithNZeros<T, H> {
    fn matches(number: T, zeros: usize) -> Option<NumberHash<T, H>>;
}
