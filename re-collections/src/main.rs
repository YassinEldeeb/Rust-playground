use std::io;
use std::{collections::HashMap, hash::Hash};

fn main() {
    let vouls = ["i", "o", "u", "e", "a"];

    let text = "first apple";
    let mut pig_latin_str = String::new();

    let words = text.split_whitespace().collect::<Vec<_>>();

    for (idx, e) in words.iter().enumerate() {
        let mut word = String::from(*e);
        let chars = word.chars().collect::<Vec<char>>();

        let first_char = String::from(chars[0]);
        if vouls.contains(&&first_char[..]) {
            word.push_str("hay");
        } else {
            word.remove(0);
            word.push_str(&(first_char + &"ay".to_string()));
        }

        pig_latin_str.push_str(&word);

        if !(idx > words.len() - 1) {
            pig_latin_str.push_str(" ");
        }
    }

    println!("{}", pig_latin_str);
}
