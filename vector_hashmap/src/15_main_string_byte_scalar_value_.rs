fn main() {
    let w = "अमिताभः";

    // 使用字节类型
    /*for b in w.bytes() {
        println!("{}", b)
    }*/

    // 使用标量值
    for c in w.chars() {
        println!("{}", c);
    }
}