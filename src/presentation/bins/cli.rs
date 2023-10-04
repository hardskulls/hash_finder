use std::sync::mpsc;

use clap::Parser;
use hash_finder::application::find_hashes::find_hashes;
use hash_finder::domain::hashing::objects::common_types::Number;
use hash_finder::presentation::apps::cli::formatting::format_num_and_hash;
use hash_finder::presentation::apps::cli::layout::CliHashFinder;

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
        let output = format_num_and_hash(num_hash);

        println!("{output}");

        counter -= 1;
        if counter == 0 {
            break;
        }
    }
}
