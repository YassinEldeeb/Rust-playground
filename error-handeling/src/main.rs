use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// fn main() {
//     a();
// }

// fn a() {
//     b();
// }

// fn b() {
//     c();
// }

// fn c() {
//     panic!("Die...just die")
// }

fn main() {
    // Nested match expressions
    // let file = File::open("hello.txt");

    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(file) => file,
    //             Err(error) => panic!("Couldn't create 'hello.txt' file: {}", error),
    //         },
    //         other_error => {
    //             panic!("Problem openning the file: {:?}", other_error)
    //         }
    //     },
    // };

    // Closures
    // let file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Couldn't create 'hello.txt' file: {}", error);
    //         })
    //     } else {
    //         panic!("Problem openning the file: {:?}", error)
    //     }
    // });

    // println!("{:?}", file);

    // Shorter syntax from the first example
    // let file = File::open("hello.txt").expect("Can't find hello.txt");
    // let file = File::open("hello.txt").unwrap();

    // A Shortcut for Propagating Errors: the ? Operator

    // fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // Ok(s)

    // Even shorter??
    // fs::read_to_string("hello.txt")
    // }

    // let username = read_username_from_file().expect("Couldn't read your username");
    // println!("{}", username);

    // Using the ? operator in main fn
    let file = File::open("hello.txt")?;
}
