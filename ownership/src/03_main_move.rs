fn main() {
    // Move

    let s1 = String::from("Hello World");
    let s2 = s1; // 这里进行了 Move 操作，转移了所有权，如果还想让s1保持有效需要用clone()函数

    // println!("{}", s1); // 这里s1已经没有所有权
    println!("{}", s2);
}