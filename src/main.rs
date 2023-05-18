#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use console::{pad_str, style, Alignment};
use dialoguer::{
    theme::{ColorfulTheme, Theme},
    Select,
};
use regex::Regex;
use std::{collections::HashMap, env, fs};

fn main() {
    let current_dir = env::current_dir().unwrap();

    // Go to texts folder
    let text_dir = current_dir.join("texts");

    if !text_dir.exists() {
        panic!(
            "{}",
            style("The 'texts' folder does not exist in the current working directory.")
                .red()
                .bold()
        );
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

    println!("\n{}\n", style("SELECT A FILE:").bold());

    let selected_file_name = Select::with_theme(&ColorfulTheme::default())
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

    for word in &valid_words {
        *word_count.entry(word.to_lowercase()).or_insert(0) += 1;
    }

    let mut hapax_legomena: Vec<String> = word_count
        .iter()
        .filter(|&(_, count)| *count == 1)
        .map(|(word, _)| word.to_lowercase())
        .collect();

    if hapax_legomena.is_empty() {
        println!(
            "\nThis text file contains {} {}.\n",
            style("no").red().bold(),
            style("Hapax Legomena").bold()
        );
        return;
    }

    println!("\n{}\n", style("HAPAX LEGOMENA:").bold());

    hapax_legomena.sort();

    for word in &hapax_legomena {
        println!("{}", word);
    }

    println!(
        "\nThere are a total of {} {}.\n",
        style(&hapax_legomena.len()).green().bold(),
        style("Hapax Legomena").bold()
    );
}
