fn main() {
    // 场景，如果我们想要获取一个变量的某个参数，而且我们对之前的变量不再关心就可以使用shadow
    // 以前的做法
    let spcaes_str = "    ";
    let spaces_len = spcaes_str.len();

    println!("The length of str is {}", spaces_len);

    // 使用shadow的做法
    let spaces = "    ";
    let spaces = spaces.len();

    println!("The length of str is {}", spaces);

    // 使用shadow使用一个变量名即可
}