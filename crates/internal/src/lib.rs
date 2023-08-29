/// Analogous to `map()` method, but works on any type, not just `Option`.
/// Convenient for chained method calls.
pub trait MapType<M, N> {
    fn map_type(self, f: impl FnOnce(M) -> N) -> N;
}

impl<M, N> MapType<M, N> for M {
    fn map_type(self, f: impl FnOnce(M) -> N) -> N {
        f(self)
    }
}

pub mod logging;

pub mod hashing {
    pub mod funcs;
    pub mod traits;
    pub mod types;
}
