use crate::application::find_hashes::find_hashes;
use crate::domain::hashing::value_objects::numbers::{Number, NumberHash};
use crate::infrastructure::logging::PeekErr;
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
