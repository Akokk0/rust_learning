// 运行被忽略的测试 cargo test -- --ignored

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 2 + 2);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert_eq!(5, 1 + 1 + 1 + 1 + 1);
    }
}