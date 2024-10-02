use std::env;
use std::process;

use wb_l2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = wb_l2::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}