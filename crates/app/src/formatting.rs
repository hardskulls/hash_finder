use colored::Colorize;
use internal::hashing::types::{Number, NumberHash};
use std::fmt::Display;

fn align_number(num_len: usize) -> String {
    let spaces = Number::MAX.to_string().len();
    String::from_iter(vec![" "; spaces - num_len])
}

pub fn print_num_and_hash(n: NumberHash<impl Display, impl Display>) {
    let number = n.number.to_string().truecolor(240, 140, 75);
    let hash = format!("\"{}\"", n.hash).truecolor(125, 190, 150);
    let alignment = align_number(number.len());
    println!("{alignment}{number}, {hash}");
}
