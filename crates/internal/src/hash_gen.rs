use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use crate::MapType;
use std::sync::mpsc;

/// Stores a number and its sha256 hash.
#[derive(Debug, Clone)]
pub struct NumberHash<I, H> {
    pub number: I,
    pub hash: H,
}

/// Returns `Option` of `NumberHash` if number's hash ends with a specified number of zeros.
pub trait HashEndsWithNZeros<T, H> {
    fn matches(number: T, zeros: usize) -> Option<NumberHash<T, H>>;
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

type Sender = mpsc::Sender<NumberHash<u128, String>>;

/// Finds hashes with a specified number of zeroes at the end, and sends them through a channel.
/// Uses `rayon` parallelization.
pub fn generate_and_send_hashes<H>(trailing_zeros: usize, sender: Sender)
where
    H: HashEndsWithNZeros<u128, String>,
{
    (1..=u128::MAX).into_par_iter().for_each(|number| {
        if let Some(num_hash) = H::matches(number, trailing_zeros) {
            sender.send(num_hash).ok();
        }
    });
}

/// If there is enough zeroes at the end of a hash returns `true`.
fn enough_zeros_at_end(hash: &str, zeros: usize) -> bool {
    let mut idx = hash.len() - 1;
    let mut zeros_left = zeros;

    while zeros_left > 0 {
        if !matches!(hash.get(idx..idx + 1), Some("0")) {
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
}
