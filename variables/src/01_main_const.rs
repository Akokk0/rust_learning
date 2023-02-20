// 常量和变量有区别，常量不可被mut关键字修饰，一旦确定就无法再修改。声明时必须指定数据类型
// 常量可以在任何作用域内进行声明，包括全局作用域
// 常量只可以绑定到常量表达式，无法绑定到函数到调用结果或只能在运行时才能计算出的值
const  MAX_POINTS: u32 = 100_000;

fn main() {
    const  MAX_POINTS: u32 = 100_000;
    println!("Hello, world!");

    // 默认使用 let 声明变量是不可变的，如果想要是变量可变则需要加上mut关键字
    let x = 5;
    println!("The value of x is {}", x);

    let mut y = 6;
    println!("The value of y is {}", y);

    y = 7;
    println!("The value of y is {}", y);
}
