use std::fmt::Debug;
use clap::Parser;


#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
pub struct Cli {
    #[clap(short='A', long, default_value = "0")]
    #[arg(group = "extra_lines")]
    pub after: usize,

    #[clap(short='B', long, default_value = "0")]
    #[arg(group = "extra_lines")]
    pub before: usize,

    #[clap(short='C', long, default_value = "0")]
    #[arg(group = "extra_lines")]
    pub context: usize,

    #[clap(short, long)]
    pub count: bool,
    
    #[clap(short, long)]
    pub ignore_case: bool,

    #[clap(short='v', long)]
    pub invert: bool,

    #[clap(short='F', long)]
    pub fixed: bool,

    #[clap(short='n', long)]
    pub line_num: bool,

    #[clap(required = true)]
    pub pattern: String,

    #[clap(required = true)]
    pub file_path: String,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            after: 0,
            before: 0,
            context: 0,
            count: false,
            ignore_case: false,
            invert: false,
            fixed: false,
            line_num: false,
            pattern: String::new(),
            file_path: String::new(),
        }
    }
}