fn main() {
    // scope

    // s 不可用
    let s = "hello"; // s 可用
                             // 可对 s 进行相关操作

} // s 作用域到此结束，s 不再可用
// 当变量走出作用域 Rust 会自动调用drop() 特殊函数