// 存在同名则需要引入父级
/*use std::fmt;
use std::io;*/

// 还可以使用别名
use std::fmt::Result;
use std::io::Result as IoResult;

/*fn f1() -> fmt::Result {}

fn f2() -> io::Result {}*/

fn f1() -> Result {}

fn f2() -> IoResult {}

fn main() {}