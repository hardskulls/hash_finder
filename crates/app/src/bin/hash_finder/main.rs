use clap::Parser;
use hash_finder::cli::CliHashFinder;
use std::sync::mpsc;
use internal::hashing::find_hashes;
use internal::hashing::types::Number;

fn main() {
    pretty_env_logger::init();
    // Create CLI.
    let cli = CliHashFinder::parse();

    let zeros = cli.expect_zeroes_at_hash_end;
    let results = cli.amount_of_results;

    let (sender, receiver) = mpsc::channel();
    // Search for hashes in background.
    std::thread::spawn(move || find_hashes(1, Number::MAX, zeros, sender));

    let mut counter = results;
    while let Ok(num_hash) = receiver.recv() {
        println!("{}, {}", num_hash.number, num_hash.hash);

        counter -= 1;
        if counter == 0 {
            break;
        }
    }
}
