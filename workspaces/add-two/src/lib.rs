pub fn add_two(num: i32) -> i32 {
    num + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(add_two(1), 3);
    }
}
