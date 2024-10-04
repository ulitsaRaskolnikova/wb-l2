use std::process;
use clap::Parser;

mod cli;

use wb_l2::cli::Cli;

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = wb_l2::run(cli) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}