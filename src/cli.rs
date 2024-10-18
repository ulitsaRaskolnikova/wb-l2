use std::fmt::Debug;
use clap::Parser;


#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(required = true)]
    pub url: String,
}