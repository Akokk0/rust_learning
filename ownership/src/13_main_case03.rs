fn main() {
    let s = String::from("Hello World");
    let word_index = first_word(&s); // 这里已经借用为不可变
    // s.clear(); // cannot borrow `s` as mutable, as it is not declared as mutable
    println!("{}", word_index);
}

// 有经验的Rust开发者会采用&str作为参数类型，因为这样就可以同时接收String和&str类型的参数了
// 定义函数时使用字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}