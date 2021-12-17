use std::collections::{hash_map, HashMap};
use std::{fmt::format, ptr::NonNull};
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    //? Vectors
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    let mut v2 = vec![1, 2, 3];

    let third = &v2[2];
    // v2.push(6);

    // println!("{}", third);

    // match v2.get(20) {
    //     Some(i) => println!("Value equals: {}", i),
    //     None => println!("Index out of bound"),
    // };

    // Strings
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    let s3 = format!("{}{}", s1, s2);

    //? Indexing strings don't work here cause rust
    // don't know in which format we want the string to be in
    let hello = String::from("नमस्ते");

    // Rust allows us to index strings but to be more specefic
    // using bytes in the String Slices

    let hello = "Здравствуйте";

    // This would return "Зд" cause each letter in russian takes 2 bytes
    let s = &hello[0..4];
    // println!("{}", s);

    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // for b in hello.bytes() {
    //     println!("{}", b);
    // }

    // Scalar values (chars)
    // for c in hello.chars() {
    //     println!("{}", c);
    // }

    // Grapheme clusters
    // ["न", "म", "स्", "ते"]
    // It's included in the standard library
    // for g in hello.graphemes(true) {
    //     println!("{}", g);
    // }

    //? Hashmaps
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 25);
    // overwritting the blue key with the new value
    scores.insert("blue".to_string(), 50);

    // Return an Entry enum
    // or_insert inserts a new entry if no entry was found
    // if there's an entity, do nothing
    scores.entry("purple".to_string()).or_insert(100);
    scores.entry("purple".to_string()).or_insert(500);

    let score = scores.get("purple");
    // println!("{:#?}", score);

    //? Hashmap example (word counter)
    let paragraph = "hello world from the world";
    let mut map = HashMap::new();

    for word in paragraph.split_whitespace() {
        let word_count = map.entry(word).or_insert(0);
        *word_count += 1;
    }

    // println!("{:#?}", map);
}
