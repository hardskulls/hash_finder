use crate::domain::hashing::abstractions::hasher::HashEndsWithNZeros;
use crate::domain::hashing::gen_range::gen_range_of_nums;
use crate::domain::hashing::objects::common_types::{Number, Sender};
use crate::domain::hashing::objects::hashers::RingHasher;
use crate::infrastructure::logging::PeekErr;

/// Finds hashes containing enough zeros at the end and sends them through channel.
pub fn find_hashes(start: Number, end: Number, with_zeros_at_end: usize, sender: Sender<Number>) {
    let filter = |n| RingHasher::matches(n, with_zeros_at_end);
    let apply = |num_and_hash| {
        sender
            .send(num_and_hash)
            .peek_err(|e| log::error!("error: {e}"))
    };
    gen_range_of_nums(start, end, filter, apply);
}
