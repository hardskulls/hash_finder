use std::sync::mpsc;

/// Stores a number and its sha256 hash.
#[derive(Debug, Clone)]
pub struct NumberHash<N, H> {
    pub number: N,
    pub hash: H,
}

pub type Number = u128;

pub type Sender<N> = mpsc::Sender<NumberHash<N, String>>;
