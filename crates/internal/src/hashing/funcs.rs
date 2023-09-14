use crate::hashing::HashEndsWithNZeros;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::hashing::hashers::RingHasher;
use crate::hashing::types::{Number, NumberHash, Sender};
use crate::logging::PeekErr;

/// Finds hashes containing enough zeros at the end and sends them through channel.
pub fn find_hashes(start: Number, end: Number, with_zeros_at_end: usize, sender: Sender) {
    let filter = |n| RingHasher::matches(n, with_zeros_at_end);
    let apply = |num_and_hash| {
        sender
            .send(num_and_hash)
            .peek_err(|e| log::error!("error: {e}"))
    };
    gen_range_of_nums(start, end, filter, apply);
}

type T<OUT> = NumberHash<Number, OUT>;

/// Generates range of numbers, filters each one and applies provided
/// function to those, which pass through the filter.  
/// Does not return anything, as it is intended to be used with things like channels.  
///
/// * Uses `rayon` for parallelization.
pub fn gen_range_of_nums<OUTPUT, F, A, RES>(start: Number, end: Number, filter: F, apply: A)
where
    F: Fn(Number) -> Option<T<OUTPUT>> + Sync + Send,
    A: Fn(T<OUTPUT>) -> RES + Sync + Send,
{
    (start..=end).into_par_iter().for_each(|number| {
        if let Some(num_hash) = filter(number) {
            apply(num_hash);
        }
    })
}

/// If there is enough zeroes at the end of a hash returns `true`.
pub(in crate::hashing) fn enough_zeros_at_end(hash: &str, zeros: usize) -> bool {
    let mut idx = hash.len() - 1;
    let mut zeros_left = zeros;

    while zeros_left > 0 {
        if !matches!(hash.get(idx..=idx), Some("0")) {
            return false;
        }
        idx -= 1;
        zeros_left -= 1;
    }

    true
}
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use pocket_micro_benching::bench_times;

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

    #[test]
    fn bench_enough_zeros() {
        let times = 1_000_000;
        let s = "765965865380986542000000000";
        let exec = || enough_zeros_at_end(s, 00000000);
        let res = bench_times(times, exec).unwrap();

        assert_eq!(res, Duration::from_millis(50));
    }
}
