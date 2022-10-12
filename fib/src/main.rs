use clap::Parser;
use color_eyre::eyre;
use std::fmt::Display;
use std::iter::successors;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Calculate fibonacci to the nth number
    #[clap(short, long, value_parser)]
    nth: usize,
}

fn fibonacci(nth: usize) {
    successors(Some((0_u128, 1_u128)), |(a, b)| Some((*b, b + a)))
        .map(|(a, _)| a)
        .take(nth)
        .for_each(println);
}

fn println<T: Display>(value: T) {
    println!("{value}")
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let nth = Args::parse().nth;
    fibonacci(nth);

    Ok(())
}
