fn main() {
    let mut s = String::from("foo");
    let s1 = String::from("bar");
    // s.push_str("bar");
    // 这里使用借用，所以并不会拿走s1的所有权，s1之后还可以使用
    s.push_str(&s1);

    println!("{}", s1);
}