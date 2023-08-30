use crate::hashing::traits::HashEndsWithNZeros;
use crate::hashing::types::{Number, NumberHash};
use crate::logging::PeekErr;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::mpsc;

pub type Sender = mpsc::Sender<NumberHash<Number, String>>;

/// Finds hashes with a specified number of zeroes at the end, and sends them through a channel.
/// Uses `rayon` parallelization.
pub fn generate_and_send_hashes<H>(trailing_zeros: usize, sender: Sender)
where
    H: HashEndsWithNZeros<Number, String>,
{
    (1..=Number::MAX).into_par_iter().for_each(|number| {
        if let Some(num_hash) = H::matches(number, trailing_zeros) {
            sender
                .send(num_hash)
                .peek_err(|err| log::error!("@[fn]:[generate_and_send_hashes]: {err:#?}"))
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
    use pocket_micro_benching::bench_times;
    use std::time::Duration;

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

    #[test]
    fn bench_enough_zeros() {
        let times = 1_000_000;
        let s = "765965865380986542000000000";
        let exec = || enough_zeros_at_end(s, 00000000);
        let res = bench_times(times, exec).unwrap();

        assert_eq!(res, Duration::from_millis(50));
    }
}
