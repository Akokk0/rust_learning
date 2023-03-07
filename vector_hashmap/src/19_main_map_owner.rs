use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    // 这里就把所有权转移给了Map
    // map.insert(field_name, field_value);
    // 这里再使用这两个变量就会报错
    // println!("{}:{}", field_name, field_value);

    // 如果这里使用借用
    map.insert(&field_name, &field_value);
    // 这里再使用这两个值就不会报错了
    println!("{}: {}", field_name, field_value);
}