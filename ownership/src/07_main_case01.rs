fn main() {
    // 不想把所有权交出去的例子

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s:String) -> (String, usize) {
    let length = s.len();

    (s, length)
}