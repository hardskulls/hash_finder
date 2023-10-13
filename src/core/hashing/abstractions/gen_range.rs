mod abstract_number;
mod simple_num_alias;

pub use abstract_number::AbstractNumber;
pub use simple_num_alias::SingleNumAlias;

/// Generates range of numbers, filters each one and applies provided
/// function to those, which pass through the filter.
/// Does not return anything, as it is intended to be used with things like channels.
pub trait GenRange<N, T, F>
where
    F: Fn(N) -> T,
{
    fn gen_range(start: N, end: N, f: F);
}
