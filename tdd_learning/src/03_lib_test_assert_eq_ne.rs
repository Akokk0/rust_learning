pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn if_adds_not_two() {
        assert_ne!(5, add_two(2));
    }
}