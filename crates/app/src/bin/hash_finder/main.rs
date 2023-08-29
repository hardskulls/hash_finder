use clap::Parser;
use hash_finder::cli::HashFinder;
use internal::hash_gen::{generate_and_send_hashes, StringSHA256Hasher};
use std::sync::mpsc;

fn main() {
    // Create CLI.
    let cli = HashFinder::parse();

    let zeros = cli.expect_zeroes_at_hash_end;
    let results = cli.amount_of_results;

    let (sender, receiver) = mpsc::channel();
    // Search for hashes in background.
    std::thread::spawn(move || generate_and_send_hashes::<StringSHA256Hasher>(zeros, sender));

    let mut counter = results;

    while let Ok(num_hash) = receiver.recv() {
        println!("{}, {}", num_hash.number, num_hash.hash);

        counter -= 1;
        if counter == 0 {
            break;
        }
    }
}
