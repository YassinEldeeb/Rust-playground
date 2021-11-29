fn main() {
    let s = String::from("Hello World, How are you?");
    let s2 = "Hello";

    let word = first_word(&s);
    let word2 = second_word(s2);

    println!("{}--{}", word, word2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut starting_index = 0;
    let mut ending_index = s.len();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && starting_index == 0 {
            starting_index = i + 1;
        } else if item == b' ' && starting_index != 0 && ending_index == s.len() {
            ending_index = i;
        }
    }

    &s[starting_index..ending_index]
}
