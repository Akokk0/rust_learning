fn main() {
    // + 拼接字符串
    let s1 = String::from("Hello,");
    let s2 = String::from("World!");

    let s3 = s1 + &s2;

    println!("{}", s3);
    println!("{}", s1); // s1不可再使用
    println!("{}", s2);
}