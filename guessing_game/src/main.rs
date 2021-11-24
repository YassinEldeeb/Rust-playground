use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number between 1 and 10 🤔");
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

        fn get_suffix(score: u32) -> String {
            return match score {
                1 => "1st".to_string(),
                2 => "2nd".to_string(),
                3 => "3rd".to_string(),
                _ => format!("{}th", score),
            };
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small! 🤏");
                tries += 1;
            }
            Ordering::Equal => {
                println!("You Win 🎉, from the {} try!", get_suffix(tries));
                break;
            }
            Ordering::Greater => {
                println!("Too big! 🚚");
                tries += 1;
            }
        }
    }
}
