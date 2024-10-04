use std::error::Error;
use std::fs;
use std::cmp;
use regex::{RegexBuilder};

pub mod cli;

pub use crate::cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&cli.file_path)?;
    let lines: Vec<&str> = contents.lines().collect();
    let pattern = if cli.fixed {
        &format!(r"^{}$", regex::escape(&cli.pattern))
    } else {
        &cli.pattern
    };
    let re = RegexBuilder::new(&pattern)
        .case_insensitive(cli.ignore_case)
        .build()
        .unwrap();

    let before = cmp::max(cli.before, cli.context);
    let after = cmp::max(cli.after, cli.context);

    let mut last = 0;
    let mut after_num = 0;

    let mut count = 0;

    for (i, line) in lines.iter().enumerate() {
        if after_num > 0 {
            after_num -= 1;
            handle_output(i, line, &mut count, &cli);
            continue;
        }
        let is_match = re.is_match(line);
        if (is_match && !cli.invert) || (!is_match && cli.invert) {
            for j in cmp::max(last, i - before)..i + 1 {
                handle_output(i, lines[j], &mut count, &cli);
            }
            last = i + 1;
            after_num = after;
        }
    }

    Ok(())
}

fn handle_output(index: usize, line: &str, count: &mut usize, cli: &Cli) {
    *count += 1;

    if cli.count {
        return;
    }

    if cli.line_num {
        println!("{} {}", index + 1, line);
    } else {
        println!("{line}");
    }
}