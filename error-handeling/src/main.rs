use std::fs::File;
use std::io::ErrorKind;

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
    let r = File::open("./hello.txt");

    let r = match r {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Couldn't create 'hello.txt' file: {}", error),
            },
            other_error => {
                panic!("Problem openning the file: {:?}", other_error)
            }
        },
    };

    println!("{:?}", r);
}
