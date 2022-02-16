use std::thread::{self};
use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    let nums = vec![
        120, 1021, 531, 592, 8693, 250, 3221, 181, 392, 6693, 120, 1021, 531, 592, 8693, 250, 3221,
        181, 392, 6693,
    ];

    let mut handles = vec![];
    for num in nums {
        let handle = thread::spawn(move || {
            let num = compute_digit(num);

            println!("Computed Result = {}", num);
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let elapsed = now.elapsed();

    println!("Performance {:?}", elapsed);
}

fn compute_digit(num: i32) -> i32 {
    for i in 0..num {
        println!("{}", i);
    }

    thread::sleep(Duration::from_secs(2));

    num + 1
}
