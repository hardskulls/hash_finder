use crate::hashing::traits::HashEndsWithNZeros;
use crate::hashing::types::NumberHash;
use crate::logging::PeekErr;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::mpsc;

pub type Sender = mpsc::Sender<NumberHash<u128, String>>;

/// Finds hashes with a specified number of zeroes at the end, and sends them through a channel.
/// Uses `rayon` parallelization.
pub fn generate_and_send_hashes<H>(trailing_zeros: usize, sender: Sender)
where
    H: HashEndsWithNZeros<u128, String>,
{
    (1..=u128::MAX).into_par_iter().for_each(|number| {
        if let Some(num_hash) = H::matches(number, trailing_zeros) {
            sender
                .send(num_hash)
                .peek_err(|err| log::error!("@[fn]:[generate_and_send_hashes]: {err:?}"))
                .ok();
        }
    });
}

/// If there is enough zeroes at the end of a hash returns `true`.
pub(super) fn enough_zeros_at_end(hash: &str, zeros: usize) -> bool {
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
