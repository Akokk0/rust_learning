// 静态变量 = 全局变量
// 静态变量只能存储'static生命周期的引用，无需显式标注
// static HELLO_WORLD: &'static str = "Hello, world!";
// 访问不可变静态变量是安全的
static HELLO_WORLD: & str = "Hello, world!";

/* 常量和不可变静态变量的区别
    - 静态变量：有固定的内存地址，使用它的值总会访问同样的数据
    - 常量：允许使用它们的时候对数据进行复制
    - 静态变量：可以是可变的，访问和修改静态可变变量是不安全的(unsafe)
*/

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    // println!("name is: {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}