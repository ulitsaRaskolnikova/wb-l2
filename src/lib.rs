use std::error::Error;
use std::fs;
use std::collections::HashMap;

mod sort;
pub mod cli;

use sort::*;
use crate::cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&cli.file_path)?;
    let mut lines: Vec<&str> = contents.lines().collect();
    let mut leading_blanks_string: HashMap<String, String> = HashMap::new();
    let old_lines = lines.clone();

    if cli.key <= 0 {
        return Err("key must be greater than 0".into());
    }
    
    if cli.ignore_leading_blanks {
        lines.iter_mut().for_each(|x| {
            let old = x.to_string();
            *x = x.trim();
            leading_blanks_string.insert(x.to_string(), old.to_string());
        });
    }
    
    sort(&mut lines, &cli);


    if cli.reverse {
        lines.reverse();
    }

    if cli.unique {
        lines.dedup();
    }

    if cli.ignore_leading_blanks {
        lines.iter_mut().for_each(|x| {
            *x = leading_blanks_string.get(&x.to_string()).unwrap();
        })
    }

    if cli.check {
        if old_lines != lines {
            println!("{:?} not equal to \n{:?}", old_lines, lines);
        }
        return Ok(());
    }

    for line in lines {
        println!("{line}");
    }

    Ok(())
}