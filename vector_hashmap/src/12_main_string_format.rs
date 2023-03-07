fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // 使用+拼接
    /*let s3 = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s3);*/

    // 使用format，而且传进去的参数后续都可以继续使用
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}