use std::{fs, time::Instant};

use practice::parse_md;

fn main() {
    let content = fs::read_to_string("test.md").unwrap();

    let results = parse_md(content);

    println!("{:#?}", results);
}
