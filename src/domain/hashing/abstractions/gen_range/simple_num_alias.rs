use crate::domain::hashing::abstractions::gen_range::GenRange;
use crate::domain::hashing::value_objects::numbers::Number;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// A `GenRange` implementor.
///
/// * Only uses a simple number alias.
/// * Uses `rayon` for parallelization.
pub struct SingleNumAlias;

impl<T, F> GenRange<Number, T, F> for SingleNumAlias
where
    F: Fn(Number) -> T + Sync + Send,
{
    fn gen_range(start: Number, end: Number, f: F) {
        let f = |n| {
            f(n);
        };
        (start..=end).into_par_iter().for_each(f)
    }
}
