// ! Never 无返回值
// 不返回值的函数也被称为发散函数(diverging function)
/*fn bar() -> ! {

}

fn main() {}*/

/*impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            // panic!的返回值是!
            None => panic!("called `Option::unwrap()` on a `None` value")
        }
    }
}*/

/*fn main() {
    let guess = "";

    loop {
        // never可以被强制转换为任何类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue // continue相当于！这里会被强制转换为num u32类型
        };
    }
}*/

fn main() {
    print!("forever");

    // loop永远不会结束，所以返回值是!
    loop {
        print!("and ever")
    }
}
