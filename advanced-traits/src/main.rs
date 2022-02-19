use std::{
    fmt::{self, Display},
    ops::{Add, Deref},
    process::Output,
};

// Associated Types
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// Default Generic Type Parameters
#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Meters(u32);

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self, border: char) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", format!("{border}").repeat(len + 4));
        println!("{border}{}{border}", " ".repeat(len + 2));
        println!("{border} {} {border}", output);
        println!("{border}{}{border}", " ".repeat(len + 2));
        println!("{}", format!("{border}").repeat(len + 4));
    }
}

impl OutlinePrint for Point {}

// Newtype Pattern
struct Wrapper(Vec<String>);

impl Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    let p = Point { x: 1, y: 3 };
    p.outline_print('*');
    // Fully Qualified Syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let person = Human;

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    assert_eq!(Millimeters(1200) + Meters(1), Millimeters(2200));

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
