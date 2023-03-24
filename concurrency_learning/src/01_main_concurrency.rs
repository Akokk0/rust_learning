// Concurrent：程序的不同部分之间独立的执行
// Parallel：程序的不同部分同时运行

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 阻塞等到线程执行结束才会走后边代码

    handle.join().unwrap();

    for i in 1..=5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap();
}

