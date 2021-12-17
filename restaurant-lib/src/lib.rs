mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    fn fix_incorrect_order() {
        super::eat_at_restaurant();
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

fn serve_order() {}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    use crate::front_of_house::hosting::add_to_waitlist;

    add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
