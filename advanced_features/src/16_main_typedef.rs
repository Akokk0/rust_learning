/*type Kilometers = i32;

fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}*/

/*fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
    // --snip--
}

fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    Box::new(|| println!("hi"))
}

fn main() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
}*/

// 优化

/*type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi"))
}

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));
}*/

// 第二个例子

use std::io::Error;
use std::fmt;

// type Result<T> = Result<T, std::io::Error>;
type Result<T> = std::io::Result<T>;

/*pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}*/

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<()>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {}
