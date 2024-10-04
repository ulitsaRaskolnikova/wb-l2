use std::collections::HashMap;


fn main() {
    let words = vec!["листок", "пятак", "дом", "ток", "тяпка", "слиток", "столик", "кот","пятка", "кто", "флекс"];
    
    let anagrams = find_anagrams(&words);
    
    println!("{:?}", anagrams);
}

fn find_anagrams(words: &[&str]) -> HashMap<String, Vec<String>> {
    let mut anagram_map: HashMap<String, Vec<String>> = HashMap::new();

    for word in words {
        let word = word.to_lowercase();
        let mut sorted_word: Vec<char> = word.chars().collect();
        sorted_word.sort();
        let key = sorted_word.iter().collect::<String>();
        anagram_map.entry(key).or_insert(Vec::new()).push(word.to_string());
    }
    
    let mut result_map: HashMap<String, Vec<String>> = HashMap::new();

    for mut anagrams in anagram_map.into_values() {
        if anagrams.len() > 1 {
            anagrams.sort();
            result_map.insert(anagrams[0].clone(), anagrams);
        }
    }

    result_map
}