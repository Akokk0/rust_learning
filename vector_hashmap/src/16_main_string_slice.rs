fn main() {
    let hello = "Привет";
    let s = &hello[0..4]; // 切割字符串时必须沿着字符边界切割

    println!("{}", s);
}