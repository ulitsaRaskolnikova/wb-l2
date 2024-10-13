use std::fmt::Debug;
use clap::Parser;


#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long, default_value = "1")]
    pub threads: usize,

    #[clap(required = true)]
    pub file_path: String,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            threads: 1,
            file_path: String::new(),
        }
    }
}