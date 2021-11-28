fn main() {
    let s = String::from("Hello World");

    let word = first_word(&s);
    let word2 = second_word(&s);

    println!("{}--{}", word, word2);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut starting_index = 0;
    let mut ending_index = s.len();
    let mut starting_index_found = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !starting_index_found {
            starting_index = i + 1;
            starting_index_found = true;
        } else if item == b' ' && starting_index_found {
            ending_index = i;
        }
    }

    &s[starting_index..ending_index]
}
