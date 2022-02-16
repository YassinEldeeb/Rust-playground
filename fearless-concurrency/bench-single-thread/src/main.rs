use bench_single_thread::parse_md;
use std::fs;

fn main() {
    let content = fs::read_to_string("test.md").unwrap();

    let results = parse_md(content);

    println!("{:#?}", results);
}
