fn main() {
    /*let _x = 5;
    let y = 10;*/

    let s = Some(String::from("Hello!"));

    /*if let Some(_s) = s { // 这里值会移动到_s
        println!("found a string");
    }*/

    if let Some(_) = s { // _不会绑定值，值不会发生移动
        println!("found a string");
    }

    println!("{:?}", s);
}