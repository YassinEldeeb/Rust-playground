fn main() {
    let value = Some(3u8);

    if let Some(max) = value {
        println!("The Value is {}", max);
    } else {
        println!("Value is None!");
    }
}
