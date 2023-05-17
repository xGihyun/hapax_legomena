#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use dialoguer::Select;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let current_dir = env::current_dir().unwrap();

    // Go to texts folder
    let text_dir = current_dir.join("texts");

    if !text_dir.exists() {
        panic!("The 'texts' folder does not exist in the current working directory.");
    }

    // Select a text file
    let paths = fs::read_dir(&text_dir).unwrap();
    let file_names: Vec<String> = paths
        .into_iter()
        .map(|path| {
            path.unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect();

    println!("SELECT A FILE:\n");

    let selected_file_name = Select::new()
        .items(&file_names)
        .default(0)
        .clear(false)
        .interact()
        .unwrap();

    let selected_file_path = text_dir.join(&file_names[selected_file_name]);
    let file = fs::read_to_string(selected_file_path).expect("Couldn't read file");

    // Hapax Legomena
    let mut word_count: HashMap<String, i32> = HashMap::new();
    let regex = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
    let words: Vec<&str> = regex.split(&file).filter(|&x| !x.is_empty()).collect();
    let valid_words: Vec<&str> = words.iter().map(|&x| x).collect();

    for word in valid_words {
        *word_count.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    println!("\nHAPAX LEGOMENA:\n");

    let mut hapax_legomena: Vec<String> = word_count
        .iter()
        .filter(|&(_, count)| *count == 1)
        .map(|(word, _)| word.to_lowercase())
        .collect();

    hapax_legomena.sort();

    for word in hapax_legomena {
        println!("{}", word);
    }
}
