mod cli;

use cli::CLI;
use clap::Parser;

fn main() {
    CLI::parse().call(module_path!())
}