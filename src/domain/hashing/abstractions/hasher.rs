use crate::domain::hashing::objects::common_types::NumberHash;

/// Returns `Option` of `NumberHash` if number's hash ends with a specified number of zeros.
pub trait HashEndsWithNZeros<NUM, HASH> {
    fn matches(number: NUM, zeros: usize) -> Option<NumberHash<NUM, HASH>>;

    fn hash_this(bytes: &[u8]) -> HASH;
}
