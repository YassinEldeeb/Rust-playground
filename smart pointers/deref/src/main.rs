use std::{
    fmt::{self, Display},
    ops::Deref,
};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Display for MyBox<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let m = MyBox::new(String::from("Yassin"));

    println!("m = {}", &m);
    hello(&m);
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
