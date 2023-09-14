use std::sync::mpsc;

/// Stores a number and its sha256 hash.
#[derive(Debug, Clone)]
pub struct NumberHash<I, H> {
    pub number: I,
    pub hash: H,
}

pub type Number = u128;

pub type Sender = mpsc::Sender<NumberHash<Number, String>>;
