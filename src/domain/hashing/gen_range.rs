use crate::domain::hashing::objects::common_types::{Number, NumberHash};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Generates range of numbers, filters each one and applies provided
/// function to those, which pass through the filter.
/// Does not return anything, as it is intended to be used with things like channels.
///
/// * Uses `rayon` for parallelization.
pub fn gen_range_of_nums<OUTPUT, F, A, RES>(start: Number, end: Number, filter: F, apply: A)
where
    F: Fn(Number) -> Option<NumberHash<Number, OUTPUT>>,
    F: Sync + Send,
    A: Fn(NumberHash<Number, OUTPUT>) -> RES,
    A: Sync + Send,
{
    (start..=end).into_par_iter().for_each(|number| {
        if let Some(num_hash) = filter(number) {
            apply(num_hash);
        }
    })
}
