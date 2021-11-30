enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    changeColor(i32, i32, i32),
}

impl Message {
    fn getTheSameInstance(&self) -> &Self {
        self
    }
    fn sayHello() -> &'static str {
        "Hello World"
    }
}

fn main() {
    let x: i8 = 5;
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_value: Option<i32> = None;

    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    println!("Hello, world!");
}
