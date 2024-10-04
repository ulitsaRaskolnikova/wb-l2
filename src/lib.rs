use std::error::Error;
use std::fs;
use std::cmp;
use regex::RegexBuilder;

pub mod cli;

pub use crate::cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&cli.file_path)?;
    let lines: Vec<&str> = contents.lines().collect();

    let (matched_lines_num, extracted_lines) = extract_lines(&lines, &cli);

    if cli.count {
        println!("{matched_lines_num}");
        return Ok(());
    }

    for (i, line) in extracted_lines.iter().enumerate() {
        if cli.line_num {
            println!("{} {}", i + 1, line);
        } else {
            println!("{line}");
        }
    }

    Ok(())
}

fn extract_lines<'a>(lines: &'a [&'a str], cli: &Cli) -> (usize, Vec<&'a str>) {
    let mut scanline: Vec<isize> = vec![0; lines.len()];

    let pattern = if cli.fixed {
        format!(r"^{}$", regex::escape(&cli.pattern))
    } else {
        cli.pattern.clone()
    };
    
    let re = RegexBuilder::new(&pattern)
        .case_insensitive(cli.ignore_case)
        .build()
        .unwrap();

    let before = cmp::max(cli.before, cli.context);
    let after = cmp::max(cli.after, cli.context);

    let mut count = 0;

    for (i, line) in lines.iter().enumerate() {
        let is_match = re.is_match(line);

        if (is_match && !cli.invert) || (!is_match && cli.invert) {
            scanline[cmp::max(0, i - before)] += 1;
            scanline[cmp::min(lines.len() - 1, i + after)] -= 1;
            count += 1;
        }
    }

    let mut scan = 0;
    let mut result_lines = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        scan += scanline[i];
        if scan > 0 || (scanline[i] == -1 && scan == 0){
            result_lines.push(*line); // Деструктурируем ссылку для правильного добавления
        }
    }

    (count,result_lines)
}

#[cfg(test)]
mod tests {
    use std::default;

    use super::*;

    fn lines() -> [&'static str; 18] {
        [
            "Rust:",
            "safe, fast,  productive.",
            "dsf",
            "df",
            "df",
            "dfs",
            "Pick three.",
            "2 b",
            "-1 a",
            "tret",
            "rt",
            "df",
            "r",
            "et",
            "wer",
            "df",
            "rt",
            "eret",
        ]
    }

    fn expected_lines() -> [&'static str; 16] {
        [
            "safe, fast,  productive.", 
            "dsf", 
            "df", 
            "df", 
            "dfs", 
            "Pick three.", 
            "2 b",
            "tret", 
            "rt", 
            "df", 
            "r", 
            "et", 
            "wer", 
            "df", 
            "rt", 
            "eret"
        ]
    }

    #[test]
    fn test_extract_lines_with_context() {
        let lines = lines();
        let (matched_lines_num, extracted_lines) = extract_lines(&lines, &Cli{
            pattern: "df".to_string(),
            context: 2,
            ..Default::default()
        });

        assert_eq!(matched_lines_num, 5);
        assert_eq!(extracted_lines, expected_lines());
    }

    #[test]
    fn test_extract_lines_with_context_and_ignore_case() {
        let lines = lines();
        let (matched_lines_num, extracted_lines) = extract_lines(&lines, &Cli{
            pattern: "DF".to_string(),
            context: 2,
            ignore_case: true,
            ..Default::default()
        });

        assert_eq!(matched_lines_num, 5);
        assert_eq!(extracted_lines, expected_lines());
    }
}