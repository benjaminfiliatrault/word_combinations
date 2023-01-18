mod helpers;
mod read_lines;
mod sanitize_data;

use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::path::Path;

use helpers::share_char;

use crate::helpers::clear_line;
use crate::sanitize_data::{remove_duplicate_words, sanitize_raw_words, sort_chars};

const RAW_DATA_FILE_PATH: &str = "./data/raw_data.txt";

fn main() {
    clear_line();

    if !Path::new(RAW_DATA_FILE_PATH).exists() {
        println!();
        println!("⛔️ Missing {}. Please create it an insert your dictionary", RAW_DATA_FILE_PATH);
        println!();
        panic!("Required file missing");
    }

    println!("Word character length?: ");
    let stdin = io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");

    let word_char_len = line.parse::<usize>().unwrap();

    println!();
    println!("How many word to find?: ");
    let stdin = io::stdin();
    let line = stdin
        .lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");

    let nb_word_to_find = line.parse::<usize>().unwrap();

    clear_line();

    println!("🧹 Cleaning data...");
    // Remove any words which are not N char long
    let cleaned_word_vec = sanitize_raw_words(RAW_DATA_FILE_PATH, word_char_len);
    // Sort the chars in every words --> abc, bac, cab => abc, abc, abc
    // Returns a hashmap of the sorted chars word and unsorted char word for word discovery
    // abc => [abc, bac, cab]
    let (hashmap, sorted_char_vec) = sort_chars(cleaned_word_vec);
    // Removes any duplicates
    let dictionary = remove_duplicate_words(sorted_char_vec);

    // Just a peekable iteration to know when we are at the last item
    let mut iteration = dictionary.iter().peekable();

    // Iterate through all the unique words file
    while let Some(current_word) = iteration.next() {
        // We treat a word as a suite of chars / an hash
        let hash = current_word.to_owned();
        // Call the function to find a word
        let current_hash = find_words(
            current_word,
            hash,
            dictionary.clone(),
            word_char_len,
            nb_word_to_find,
        );

        print_info(current_word, &current_hash, word_char_len, nb_word_to_find);

        // We have X chars words and we want to find Y words so
        if current_hash.chars().count() >= (word_char_len * nb_word_to_find).try_into().unwrap() {
            print_found_result(&current_hash, hashmap, word_char_len);
            return;
        }

        if iteration.peek().is_none() {
            clear_line();
            println!();
            println!("NOTHING WAS FOUND! 😢")
        }
    }

    //* Entering recursive function
    fn find_words(
        current_word: &String,
        current_hash: String,
        mut dictionary: HashSet<String>,
        word_char_len: usize,
        nb_word_to_find: usize,
    ) -> String {
        if current_hash.chars().count() >= (word_char_len * nb_word_to_find).try_into().unwrap() {
            return current_hash;
        }
        // Find a suitable candidate in the dictionary
        // the candidate mustn't share chars with the current hash
        let candidate = dictionary
            .clone()
            .into_iter()
            .find(|word| !share_char(&current_hash, word));

        if candidate.is_some() {
            // If we have a candidate, unwrap the result
            let candidate = candidate.unwrap();
            // Append the candidate to the hash
            let current_hash = current_hash.clone() + &candidate.to_owned();

            // Pop the word out of the dictionary
            dictionary.remove(&candidate);

            // Call the function again to find another candidate
            return find_words(
                current_word,
                current_hash,
                dictionary,
                word_char_len,
                nb_word_to_find,
            );
        }
        return current_hash;
    }
}

fn print_info(
    current_word: &String,
    current_hash: &String,
    word_char_len: usize,
    nb_word_to_find: usize,
) {
    clear_line();
    println!("🔎 Searching a combination...");
    println!();
    println!();
    println!("📋 Current Parameters");
    println!("------------------------");
    println!("Character length: {}", word_char_len);
    println!("Number of word to find: {}", nb_word_to_find);
    println!();
    println!();
    println!("📚 Current data");
    println!("------------------------");
    println!("Current word: {:?}", current_word);
    println!("Current hash: {:?}", current_hash);
}

fn print_found_result(
    current_hash: &String,
    hashmap: HashMap<String, Vec<String>>,
    word_char_len: usize,
) {
    clear_line();

    // Split the hash at X chars length
    let subs = current_hash
        .as_bytes()
        .chunks(word_char_len)
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    println!();

    println!(
        "{} words of a length of {} which doesn't share character were found: ",
        subs.iter().count(),
        word_char_len
    );
    println!();

    for (idx, sub) in subs.into_iter().enumerate() {
        println!("{}. {:?}", idx + 1, hashmap.get(sub).unwrap()[0]);
    }

    println!();
}
