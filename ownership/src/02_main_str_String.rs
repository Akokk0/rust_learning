fn main() {
    // String 类型，为了支持可变性，需要在heap上分配内存来保存编译时未知的文本内容
    let mut s = String::from("Hello");

    s.push_str(", World");

    println!("{}", s);

    // 字符串字面值，在编译时就知道它的内容，其文本内容直接被硬编码到最终的可执行文件里
    // 速度快、高效。是因为其不可变性
    let a = "hello world";
}