use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); // 所有权已被移动
    });

    // recv() 一直阻塞线程，直到有消息传入
    // try_recv() 方法：不会阻塞，立即返回Result<T, E>
    // 通常会使用循环调用来检查try_recv()的结果
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}