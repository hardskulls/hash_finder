/// Stores a number and its hash.
#[derive(Debug, Clone)]
pub struct NumberHash<N, H> {
    pub number: N,
    pub hash: H,
}

pub type Number = u128;
