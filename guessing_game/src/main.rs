use std::io; // prelude
use std::cmp::Ordering;
use rand::Rng; // trait 类似其他语言的接口 interface

fn main() {
    println!("猜数！");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("神秘数字是：{}", secret_number);

    println!("猜测一个数");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字！");
                continue
            }
        };

        println!("你猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
            Ordering::Greater => println!("Too Big!")
        }
    }
}
