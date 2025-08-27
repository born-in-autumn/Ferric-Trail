mod core;
mod cli;
use clap::Parser;
use cli::command::handle_cli;

use crate::cli::args::Cli;

fn main() {
    let args: Cli = Cli::parse();
    println!("args: {:?}", args);
    handle_cli(args);
}
