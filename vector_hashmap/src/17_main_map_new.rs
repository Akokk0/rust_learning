use std::collections::HashMap;

fn main() {
    // Type annotations needed for `HashMap<K, V>`
    // 必须显式指明它的值
    // let mut scores: HashMap<String, i32> = HashMap::new();
    let mut scores = HashMap::new();

    // 也可以直接给他值就不用显式指明它的数据结构
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
}