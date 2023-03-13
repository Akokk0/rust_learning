fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2); // 这里result的生命周期为string2的生命周期
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    let result = String::from("abc");
    // 产生了悬垂引用
    result.as_str()
}
