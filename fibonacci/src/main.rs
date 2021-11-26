use std::io;

fn main() {
    let mut num = String::new();

    println!("Get the n-th Fibonacci number");
    io::stdin()
        .read_line(&mut num)
        .expect("Could't read the Input!");

    let num: i32 = num.trim().parse().expect("Not a number!");

    println!(
        "The {} Fibonacci number is: {}",
        get_suffix(num),
        fibonacci(num)
    );
}

fn fibonacci(num: i32) -> i32 {
    if num <= 0 {
        panic!("Please Enter a number higher than 0!");
    } else if num == 1 {
        return 0;
    } else if num == 2 {
        return 1;
    } else {
        return fibonacci(num - 1) + fibonacci(num - 2);
    }
}

pub fn get_suffix(score: i32) -> String {
    return match score {
        1 => "1st".to_string(),
        2 => "2nd".to_string(),
        3 => "3rd".to_string(),
        _ => format!("{}th", score),
    };
}
