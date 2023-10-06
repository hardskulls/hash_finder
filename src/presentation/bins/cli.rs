use std::sync::mpsc;

use clap::Parser;
use hash_finder::presentation::apps::cli::formatting::format_num_and_hash;
use hash_finder::presentation::apps::cli::functionality::search_for_hasher_in_bg;
use hash_finder::presentation::apps::cli::layout::CliHashFinder;

fn main() {
    pretty_env_logger::init();
    // Create CLI.
    let cli = CliHashFinder::parse();

    let (zeros, results) = (cli.expect_zeroes_at_hash_end, cli.amount_of_results);

    let (sender, receiver) = mpsc::channel();
    search_for_hasher_in_bg(zeros, sender);

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
