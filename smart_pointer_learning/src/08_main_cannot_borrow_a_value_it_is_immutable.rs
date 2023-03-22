fn main() {
    let x = 5;
    // 内部可变性：可变的借用一个不可变的值
    let y = &mut x;
}