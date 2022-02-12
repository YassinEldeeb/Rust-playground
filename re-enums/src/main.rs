fn main() {
    let age = Some(17);

    if let Some(num) = age {
        println!("{}", num);
    }
}
