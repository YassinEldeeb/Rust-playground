#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 5,
        height: 3,
    };

    let rect2 = Rectangle {
        width: 4,
        height: 2,
    };

    dbg!(&rect1);

    println!("The Area of the Rectangle is {}", rect1.area());
    println!("Can Hold {}", rect1.can_hold(&rect2));
    println!("Associated Function {:#?}", Rectangle::square(2));
}
