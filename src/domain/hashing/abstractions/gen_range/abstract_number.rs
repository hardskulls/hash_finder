use crate::domain::hashing::abstractions::gen_range::GenRange;
use num_traits::{Num, ToPrimitive};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::ops::{Add, RangeInclusive};

/// A `GenRange` implementor.
///
/// * Uses `num_traits` for number.
/// * Uses `rayon` for parallelization.
pub struct AbstractNumber;

fn splitter<N>(data: RangeInclusive<N>) -> (RangeInclusive<N>, Option<RangeInclusive<N>>)
where
    N: Num + Copy,
{
    let (start, end) = (*data.start(), *data.end());
    let middle = end.div(N::one() + N::one());
    let next = middle + N::one();
    (start..=middle, Some(next..=end))
}

impl<N, T, F> GenRange<N, T, F> for AbstractNumber
where
    N: Num + ToPrimitive + Add<N, Output = N> + PartialOrd + Copy + Send,
    F: Fn(N) -> T + Sync + Send,
{
    fn gen_range(start: N, end: N, f: F) {
        rayon::iter::split(start..=end, splitter)
            .into_par_iter()
            .for_each(|sub_range| {
                let (start, end) = (*sub_range.start(), *sub_range.end());
                let f = |n| {
                    f(n);
                };
                num_iter::range_inclusive(start, end).for_each(f)
            })
    }
}
