fn main() {
    // 可以使用to_string方法和from两种方法创建String
    let data = "initial contents";
    let s = data.to_string();

    let s1 = "initial contents".to_string();

    // 使用from方法，因为使用utf-8编码，所有语言都是可以支持的
    let hello = String::from("你好");
    let hello = String::from("こんにちは");
}