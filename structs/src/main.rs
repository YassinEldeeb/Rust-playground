struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}

struct Color(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let my_account = sign_up_user(
        String::from("yassineldeeb94@gmail.com"),
        String::from("Yassin"),
        17,
    );

    let _red = Color(255, 255, 255);
    let _black = Color(0, 0, 0);
    let _unit_type = AlwaysEqual;

    let user2 = User {
        email: String::from("another@example.com"),
        ..my_account
    };

    println!("Hello, {}!", user2.username);
}

fn sign_up_user(email: String, username: String, age: u8) -> User {
    User {
        username,
        email,
        age,
        active: true,
    }
}
