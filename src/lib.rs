use std::{error::Error, fs, thread, sync::{Arc, Mutex}, time::Instant};

pub mod cli;
pub use crate::cli::Cli;

const LETTER_A: u8 = 'a' as u8;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let now = Instant::now();

    let contents = fs::read_to_string(&cli.file_path)?;

    let letters: Arc<Mutex<[usize; 26]>> = Arc::new(Mutex::new([0; 26]));
    let block = contents.len() / cli.threads;

    let mut handles = Vec::new();
    let contents_arc = Arc::new(contents); // Оборачиваем contents в Arc для безопасного доступа

    for i in 0..cli.threads {
        let left = i * block;
        let right = if i == cli.threads - 1 { contents_arc.len() } else { (i + 1) * block };

        let letters = Arc::clone(&letters);
        let contents = Arc::clone(&contents_arc); // Создаем клон содержимого для каждого потока

        let handle = thread::spawn(move || {
            let mut letters = letters.lock().unwrap();
            letter_frequency(&contents[left..right], &mut letters);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed = now.elapsed().as_micros();

    println!("{}", letter_frequency_to_json(&letters.lock().unwrap(), elapsed));

    Ok(())
}

fn letter_frequency(contents: &str, letters: &mut [usize; 26]) {
    contents
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .for_each(|letter| letters[(letter as u8 - LETTER_A) as usize] += 1);
}

fn letter_frequency_to_json(&letters: &[usize; 26], elapsed: u128) -> String {
    format!("{{\n\t\"elapsed\": \"{elapsed} ms\",\n\t\"result\": {{\n{}\t}} \n}}\n",
        letters.iter()
            .enumerate()
            .map(|(letter, frequency)| 
                format!("\t\t\"{}\": {}{}\n", (letter as u8 + LETTER_A) as char, frequency, 
                (if (letter as u8 + LETTER_A) as char != 'z' {","} else {""}).to_string()))
            .collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_frequency() {
        let contents = "aAaAa1234   bBBB";
        let mut letters = [0; 26];
        letter_frequency(contents, &mut letters);
        assert_eq!(letters, [5, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}