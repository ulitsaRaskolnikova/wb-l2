use std::error::Error;
use std::fs;

pub struct Config {
    pub option: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 1 {
            return Err("not enough arguments");
        }

        if args.len() > 3 {
            return Err("extra arguments");
        } 

        if args.len() == 2 {
            let file_path = args[1].clone();
            return Ok(Config { option: "-w".to_string(), file_path });
        }

        let option = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { option, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = match config.option.as_str() {
        "-w" => count_words(&contents),
        "-c" => count_chars(&contents),
        "-l" => count_lines(&contents),
        _ => return Err("invalid option".into())
    };

    println!("{}", result);

    Ok(())
}

pub fn count_words(contents: &str) -> usize {
    let mut word_count = 0;

    for line in contents.lines() {
        word_count += line
            .split_whitespace()
            .filter(|word| word.len() > 0)
            .count();
    }

    word_count
}

pub fn count_lines(contents: &str) -> usize {
    contents.lines().count()
}

pub fn count_chars(contents: &str) -> usize {
    contents.chars().count()
}


#[cfg(test)]
mod tests {
    use super::*;

    fn contents() -> &'static str {
        "\
Rust:
safe, fast, productive.
Pick three."
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words(contents()), 6);
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines(contents()), 3);
    }

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars(contents()), 41);
    }
}
