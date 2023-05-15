use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file = fs::read_to_string("D:/Project/Programming/Rust/hapax_legomena/src/text.txt")
        .expect("Couldn't read file");
    let mut word_count: HashMap<String, i32> = HashMap::new();
    let regex = Regex::new(r"[[:punct:]\s]+").unwrap();
    let words: Vec<&str> = regex.split(&file).filter(|&x| !x.is_empty()).collect();

    for word in words {
        let count = word_count
            .entry(word.to_lowercase())
            .or_insert(0);
        *count += 1;
    }

    println!("HAPAX LEGOMENA\n");

    for (word, count) in &word_count {
        if *count == 1 {
            // println!("{}: {}", word, count);
            println!("{}", word);
        }
        // println!("{}: {}", word, count);
    }
}
