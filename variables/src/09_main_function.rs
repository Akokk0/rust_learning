fn main() {
    // 函数
    // 依照惯例，针对函数和变量名，Rust使用 snake case 命名规范：
    // 所有的字母都是小写的，单词之间使用下划线分开

    println!("hello world");
    another_function();
    parameter_function(17, 9); // argument

    // 函数返回值
    let x = plus_five(6);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function");
}

// parameter(行参) argument(实参)
fn parameter_function(x: i32, y: i32) { // parameter
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
}

// 函数体中的语句(statement)与表达式(expression)
fn statement_expression() {
    // 这是一个语句
    let x = 5;

    let y = { // 这一个代码块为一个表达式
        let x = 1;
        // x + 3; // 若在结尾加上一个分号则代表一个语句，语句的返回值是一个空的Tuple ()
        x + 3 // 作为该表达式的返回值
    };

    println!("The value of y is: {}", y);
} 

// 函数返回值
fn plus_five(x: i32) -> i32 {
    // 默认函数最后一个表达式为返回值，若想提前返回则使用return关键字
    x + 5 // 没有带 ; 则表示为表达式，作为函数返回值
}