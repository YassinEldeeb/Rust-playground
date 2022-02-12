pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_one() {
        assert_eq!(add_one(1), 2);
    }
}
