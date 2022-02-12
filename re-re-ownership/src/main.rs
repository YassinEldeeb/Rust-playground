#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 100,
        height: 60,
    };
    let square = Rectangle::square(10);
    println!("{:#?}", square);
}
