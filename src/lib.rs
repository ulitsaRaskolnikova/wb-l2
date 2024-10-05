use std::{error::Error, fs};

pub mod cli;

pub use crate::cli::Cli;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&cli.file_path)?;

    let cut_contents = cut_by_delimiter(&contents, &cli);

    for line in cut_contents {
        println!("{line}");
    }
 
    Ok(())
}

fn cut_by_delimiter(contents: &str, cli: &Cli) -> Vec<String> {
    let mut result = Vec::new();

    for line in contents.lines() {
        let separated_line: Vec<&str> = line.split(cli.delimiter).collect();
        let mut no_field = true;

        let mut cut_line = String::new();

        for field in &cli.fields {
            let field = *field - 1;
            if field < separated_line.len() {
                cut_line.push_str(separated_line[field]);
                if field < separated_line.len() - 1 {
                    cut_line.push(cli.delimiter);
                }
                no_field = false;
            }
        }

        if no_field {
            if cli.separated {
                continue;
            }
            cut_line = line.to_string();
        } 

        result.push(cut_line);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cut_by_delimiter_default() {
        let contents = "a\tb\tc\n1\t2\t3";
        let cli = Cli::default();
        let expected = vec!["a\tb\tc", "1\t2\t3"];
        assert_eq!(cut_by_delimiter(contents, &cli), expected);
    }

    #[test]
    fn test_cut_by_delimiter_separated() {
        let contents = "a\tb\tc\n1\t2\t3";
        let cli = Cli {
            separated: true,
            ..Default::default()
        };
        let expected: Vec<String> = Vec::new();
        assert_eq!(cut_by_delimiter(contents, &cli), expected);
    }

    #[test]
    fn test_cut_by_delimiter_fields() {
        let contents = "a\tb\tc\n1\t2\nflex";
        let cli = Cli {
            fields: vec![2, 3],
            ..Default::default()
        };
        let expected = vec!["b\tc", "2", "flex"];
        assert_eq!(cut_by_delimiter(contents, &cli), expected);
    }

    #[test]
    fn test_cut_by_delimiter_fields_separated() {
        let contents = "a\tb\tc\n1";
        let cli = Cli {
            fields: vec![2, 3],
            separated: true,
            ..Default::default()
        };
        let expected = vec!["b\tc"];
        assert_eq!(cut_by_delimiter(contents, &cli), expected);
    }

    
    #[test]
    fn test_cut_by_delimiter_comma() {
        let contents = "a,b,c\n1,2,3";
        let cli = Cli {
            fields: vec![2, 3],
            delimiter: ',',
            ..Default::default()
        };
        let expected = vec!["b,c", "2,3"];
        assert_eq!(cut_by_delimiter(contents, &cli), expected);
    }
}
