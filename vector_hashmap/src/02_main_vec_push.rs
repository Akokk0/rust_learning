fn main() {
    let mut v = Vec::new();

    // 这条语句加之前是Vec<?>类型，加之后是Vec<i32>类型，编译器可以自动推断数据类型
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
}