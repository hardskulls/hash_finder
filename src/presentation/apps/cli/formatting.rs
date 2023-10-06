use crate::domain::hashing::value_objects::numbers::NumberHash;
use colored::Colorize;
use num_traits::{Bounded, Num};
use std::fmt::Display;

fn align_number<N>(num_len: usize) -> String
where
    N: Bounded + Display,
{
    let spaces = N::max_value().to_string().len();
    String::from_iter(vec![" "; spaces - num_len])
}

pub fn format_num_and_hash<N>(n: NumberHash<N, impl Display>) -> String
where
    N: Num + Display + Bounded,
{
    let number = n.number.to_string().truecolor(240, 140, 75);
    let hash = format!("\"{}\"", n.hash).truecolor(125, 190, 150);

    let alignment = align_number::<N>(number.len());

    format!("{alignment}{number}, {hash}")
}
