#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Egp,
    Quarter(UsState),
}

fn main() {
    let five = Some(5);
    let six = add_one(five);
    let none = add_one(None);

    println!("{:?}", none);
    println!("{:?}", six);

    println!("{}", value_in_cents(Coin::Egp));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
        _ => {
            println!("Special Coin");
            100
        }
    }
}

fn add_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(i) => Some(i + 1),
    }
}
