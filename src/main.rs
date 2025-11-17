#![doc = include_str!("../README.md")]

use clap::Parser as _;

mod cli;

fn main() {
    cli::Cli::parse();
    println!("Hello, world!");
}
