use std::ops::Deref;

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // &m &MyBox<String>
    // deref &String
    // deref &str
    hello(&m);
    hello(&(*m)[..]);

    hello("Rust");
}

// 元组结构体
struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}