fn main() {
    // copy

    // x的值在编译时就已经确定，可以完整的存储在stack中，所以深拷贝和浅拷贝没有任何区别
    // 对于标量类型就可以直接使用copy不会造成原来的变量失效
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // Copy trait，可以用于像整数这样完全存放在stack上面的类型
    // 如果一个类型实现了Copy这个 trait ，那么旧的变量在赋值后仍然可用
    // 如果一个类型或者该类型的一部分实现了Drop trait，那么Rust不允许让它再去实现Copy trait了

    /* 一些拥有Copy trait的类型
        任何简单标量的组合类型都可以是Copy的
        任何需要分配内存或某种资源的都不是Copy的
        一些用于Copy trait的类型：
            - 所有的整数类型，例如u32
            - bool
            - char
            - 所有的浮点类型，例如f64
            - Tuple(元组)，如果其所有字段都是Copy的
                (i32, i32) 是
                (i32, String) 不是
     */
}