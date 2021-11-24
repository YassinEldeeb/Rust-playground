use rand::Rng;
use std::cmp::Ordering;
use std::io;
mod utils;

fn main() {
    utils::intro::print_intro();
    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut tries = 0;

    loop {
        println!("Please Input your guess 👇");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your Input!");

        if guess.trim() == "quit" {
            println!("noob 😒");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a 🔢!");
                continue;
            }
        };

        tries += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! 🤏");
            }
            Ordering::Equal => {
                println!(
                    "You Win 🎉, from the {} try!",
                    utils::format::get_suffix(tries)
                );
                break;
            }
            Ordering::Greater => {
                println!("Too big! 🚚");
            }
        }
    }
}
