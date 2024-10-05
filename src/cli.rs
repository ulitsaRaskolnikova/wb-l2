use std::fmt::Debug;
use clap::Parser;


#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long, default_value = "\t")]
    pub delimiter: char,

    #[clap(short, long)]
    pub separated: bool,

    #[clap(short, long, required = true, value_delimiter = ',')]
    pub fields: Vec<usize>,

    #[clap(required = true)]
    pub file_path: String,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            delimiter: '\t',
            separated: false,
            file_path: String::new(),
        }
    }
}