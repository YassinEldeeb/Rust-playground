#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 3,
    };

    dbg!(&rect);

    println!("The Area of the Rectangle is {}", rect.area());
}
