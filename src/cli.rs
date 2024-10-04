use std::fmt::Debug;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(disable_help_flag = true)]
pub struct Cli {
    #[clap(short, long, default_value = "1")]
    pub key: usize,

    #[clap(short, long)]
    #[arg(group = "type")]
    pub numeric_sort: bool,

    #[clap(short, long)]
    pub reverse: bool,

    #[clap(short, long)]
    pub unique: bool,

    #[clap(short='M', long)]
    #[arg(group = "type")]
    pub month_sort: bool,

    #[clap(short, long)]
    pub check: bool,

    #[clap(short='b', long)]
    pub ignore_leading_blanks: bool,

    #[clap(short, long)]
    #[arg(group = "type")]
    pub human_numeric_sort: bool,

    #[clap(required = true)]
    pub file_path: String,

    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}