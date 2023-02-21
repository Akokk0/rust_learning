fn main() {
    let s = String::from("Hello World");

    take_ownership(s); // s 移动到函数里，从这里开始失效

    let x = 5;

    makes_copy(x); // x 移动到函数里，但是 x 是i32类型，实际上是x的副本移动进去了

    println!("x: {}", x); // x 仍然有效
} // s 和 x 移动出了作用域就不再有效，s之前就已经发送了移动所以不会有什么特别的事发生

fn take_ownership(some_string: String) { // some_string从这里进入作用域
    println!("{}", some_string);
} // some_string从这里离开作用域，会自动调用drop()函数

fn makes_copy(some_number: i32) { // some_number从这里进入作用域，传进来的其实是副本
    println!("{}", some_number);
} // some_number离开了作用域，对于这样的类型不会有特别的事发生