fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 这是一个不可变借用
    // v.push(6); // 这是一个可变借用
    // 同一作用域下不能同时出现可变引用和不可变引用
    println!("The first element is {}", first);
}