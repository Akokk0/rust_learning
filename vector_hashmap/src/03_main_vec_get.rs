fn main() {
    let v = vec![1, 2, 3, 4, 5];
    // 如果访问索引超出范围，程序会panic
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // get方式则会返回None
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element")
    }
}