use clap::Parser;

/// Program that finds F results of natural number hashes with N zeroes at the end of the hash.
/// Note that when N is 7 or greater, the algorithm slows down significantly.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(help_template = "\n\
    \n----------------------------------------------------------------------\
    \n\n{author-with-newline}\
    Version: {version}
    \n{about-section} {usage-heading} {usage} \
    \n\n {all-args} {tab}\
    ")]
pub struct CliHashFinder {
    /// Specifies how many zeroes should be at the end of a hash.
    #[arg(short = 'N', value_name = "ZEROS")]
    pub expect_zeroes_at_hash_end: usize,
    /// Specifies how many results are expected.
    #[arg(short = 'F', value_name = "RESULTS")]
    pub amount_of_results: usize,
}
