use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        // lock会阻塞直到获得锁
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}