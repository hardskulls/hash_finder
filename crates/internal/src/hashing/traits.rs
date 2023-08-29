use crate::hashing::types::NumberHash;

/// Returns `Option` of `NumberHash` if number's hash ends with a specified number of zeros.
pub trait HashEndsWithNZeros<IN, OUT> {
    fn matches(number: IN, zeros: usize) -> Option<NumberHash<IN, OUT>>;
    
    fn hash_this(bytes: &[u8]) -> OUT;
}
