use rand::Rng;
use std::cmp::Ordering;
use std::io;
mod intro;
mod utils;

fn main() {
    intro::print_intro();
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut tries = 0;

    loop {
        println!("Please Input your guess 👇");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your Input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a 🔢!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! 🤏");
                tries += 1;
            }
            Ordering::Equal => {
                println!("You Win 🎉, from the {} try!", utils::get_suffix(tries));
                break;
            }
            Ordering::Greater => {
                println!("Too big! 🚚");
                tries += 1;
            }
        }
    }
}
