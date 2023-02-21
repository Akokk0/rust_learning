fn main() {
    // 返回值与作用域
    // 函数在返回值的过程中同样也会发生所有权的转移

    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
} // s2之前已经Move了，所以不会有特别的事发生，s3,s1则会调用drop()函数

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回值的所有权移动
}

// 取得了 String 的所有权并返回该所有权
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

/* 一个变量的所有权总是遵循同样的模式：
    - 把一个值值赋给其他变量时就会发生移动
    - 当一个包含 heap 数据的变量离开作用域时，它的值就会被drop()清理，除非数据的所有权移动到另一个变量上
*/