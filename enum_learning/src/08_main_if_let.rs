// if let
fn main() {
    let v = Some(0u8);
    match v {
        Some(3) => println!("three"),
        _ => println!("others")
    }

    /*if let Some(3) = v {
        println!("three")
    }*/
    /* if let
        · 处理只关心一种匹配而忽略其他匹配的情况
        · 更少的代码，更少的缩进，更少的模版代码
        · 放弃了穷举的可能
        · 可以把if let 看作是match的语法糖
        · 还可以搭配else进行使用
    */
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }
}