use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    /*// 这段代码相当于下面那段代码
    let mut f = File::open("hello.txt")?;
    /*let mut f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e)
    };*/

    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    /*match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }*/*/

    // 还可以更精简改成链式调用
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s);
}

fn main() {
    let result = read_username_from_file();
}