fn main() {
    let x = 5;
    // x = x + 1 这样是错误的，x是不可变的变量
    // 再次使用 let 将之前的 x shadow(隐藏)掉，之后代码的x都由新x代替
    let x = x + 1;

    println!("The value of x is {}", x);
}