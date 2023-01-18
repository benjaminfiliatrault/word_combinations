use regex::{Regex};
use std::collections::{HashMap, HashSet};

use crate::read_lines::read_lines;

// const CACHE_PATH: &str = ".cache";

pub fn sanitize_raw_words(file_path: &str, char_length: usize) -> Vec<String> {
    let mut queue_word: Vec<String> = Vec::new();

    // if Path::new(CACHE_PATH).exists() {
    //     println!();
    //     println!("⛔️ Missing {}. Please create it an insert your dictionary", CACHE_PATH);
    //     println!();
    //     panic!("Required file missing");
    // }

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let mut word = ip.to_lowercase().clone();
                let string_char_length = ip.chars().count();

                // It matches anything but a-z.
                let regex = Regex::new(r"[^a-z]+").unwrap();

                if string_char_length == char_length {
                    let mut has_same_char = false;

                    for (idx, char) in ip.chars().enumerate() {
                        let mut char_vec: Vec<char> = ip.chars().into_iter().collect();
                        if has_same_char {
                            continue;
                        };

                        char_vec.remove(idx);

                        if char_vec.contains(&char) {
                            has_same_char = true;
                            continue;
                        }
                    }

                    // No same chars and make sure we don't have duplicates
                    if !has_same_char && !regex.is_match(word.as_str()) && !queue_word.contains(&ip.to_lowercase()) {
                        queue_word.push(ip.to_lowercase());
                    }
                }
            }
        }
    } else {
        panic!("Cannot read file");
    }

    return queue_word;
}

pub fn sort_chars(words: Vec<String>) -> (HashMap<String, Vec<String>>, Vec<String>) {
    let mut queue_word: Vec<String> = Vec::new();
    let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

    // Consumes the iterator, returns an (Optional) String
    for word in words {
        let word_slice: &str = &word[..];
        let mut word_chars: Vec<char> = word_slice.chars().collect();
        word_chars.sort_by(|a, b| a.cmp(b));
        let sorted_word = String::from_iter(word_chars);

        hashmap
            .entry(sorted_word.to_string())
            .or_insert(Vec::new())
            .push(word.to_string());

        queue_word.push(sorted_word);
    }

    return (hashmap, queue_word);
}

pub fn remove_duplicate_words(mut words: Vec<String>) -> HashSet<String> {
    words.sort();
    words.dedup();
    return HashSet::from_iter(words.iter().cloned());
}
