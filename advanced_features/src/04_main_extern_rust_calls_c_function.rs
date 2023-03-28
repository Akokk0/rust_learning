// extern 关键字：简化创建和使用外部函数接口(FFI)的过程
// 外部函数接口(FFI, Foreign Function Interface)：它允许一种编程语言定义函数，并让其他编程语言能调用这些函数
// "C" 代表应用二进制接口(ABI, Application Binary Interface)：定义函数在汇编层的调用方式
// "C" ABI是最常见的ABI，它遵循C语言的ABI
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}