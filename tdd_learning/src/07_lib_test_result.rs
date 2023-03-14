#[cfg(test)]
mod tests {
    use super::*;

    // 注意：不要在使用Result<T, E>编写的测试上标注#[should_panic]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}