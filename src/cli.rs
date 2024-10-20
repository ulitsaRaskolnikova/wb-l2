use std::fmt::Debug;
use clap::Parser;


#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(long, default_value = "10")]
    pub timeout: u64,
    #[clap(required = true)]
    pub host: String,
    #[clap(required = true)]
    pub port: u16,
}