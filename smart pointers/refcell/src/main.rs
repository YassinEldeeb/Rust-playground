pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &'a T, max: usize) -> Self {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("You're over your quota!")
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("You've used over 90% of your quota!")
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("You've used over 75% of your quota!")
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    struct MockedMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockedMessenger {
        fn new() -> MockedMessenger {
            MockedMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockedMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    use super::*;
    use std::cell::RefCell;
    #[test]
    fn it_sends_a_message_on_75_usage() {
        let mock_messenger = MockedMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
