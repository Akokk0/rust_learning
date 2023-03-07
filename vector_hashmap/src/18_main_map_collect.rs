use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 这里必须要指明需要的数据类型，两个下划线因为Rust编译器可以自动推导出来
    let scores: HashMap<_, _> =
        teams.iter().zip(initial_scores.iter()).collect();
}