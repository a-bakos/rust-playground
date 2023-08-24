use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    // Read file and build vector of individual words
    let contents = match env::args().nth(1) {
        Some(f) => match fs::read_to_string(f) {
            Ok(s) => s.to_lowercase(),
            Err(e) => {
                eprintln!("Could not read file: {}", e);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Could not read file");
            std::process::exit(2);
        }
    };

    // split returns an iterator so use the collect method to
    // turn the iterator to a vector of string slices
    // collect is useful to put item from an iterator into a
    // collection
    let all_words = contents.split_whitespace().collect::<Vec<&str>>();

    let mut word_counts: HashMap<&str, u32> = HashMap::new();
    for word in all_words.iter() {
        *word_counts.entry(word).or_insert(1) += 1;
    }

    // determine the most commonly used words
    let mut top_count = 0u32;
    let mut top_words: Vec<&str> = Vec::new();
    for (&key, &val) in word_counts.iter() {
        if val > top_count {
            top_count = val;
            top_words.clear();
            top_words.push(key);
        } else if val == top_count {
            top_words.push(key);
        }
    }

    // display result
    println!("Top word(s) occurred {} times: ", top_count);
    for word in top_words {
        println!("{}", word);
    }
}
