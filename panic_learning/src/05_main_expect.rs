use std::fs::File;
use std::io::ErrorKind;

fn main() {
    /*let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Error opening file {:?}", error)
        }
    };*/

    // 这条代码相当于上面的代码
    // let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("无法打开文件 hello.txt");
}