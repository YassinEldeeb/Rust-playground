mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    fn fix_incorrect_order() {
        super::serve_order();
    }
}

fn serve_order() {}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
