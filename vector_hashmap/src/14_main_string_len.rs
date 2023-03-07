fn main() {
    let len = String::from("Hola").len();

    // Unicode标量值，有些时候一个字符包含几个字节
    println!("{}", len);
}