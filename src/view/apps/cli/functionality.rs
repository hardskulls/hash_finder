use crate::core::hashing::types::numbers::{Number, NumberHash};
use crate::external::logging::PeekErr;
use crate::usecases::find_hashes::find_hashes;
use std::sync::mpsc;

pub type Sender<N> = mpsc::Sender<NumberHash<N, String>>;

pub fn search_for_hasher_in_bg(zeros: usize, sender: Sender<Number>) {
    let send_hashes = move |num_hash| {
        sender
            .send(num_hash)
            .peek_err(|e| log::error!("error: {e}"))
            .ok();
    };
    std::thread::spawn(move || find_hashes(1, Number::MAX, zeros, send_hashes));
}
