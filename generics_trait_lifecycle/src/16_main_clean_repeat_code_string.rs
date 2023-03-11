fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // 模式匹配
    for item in list {
        if item > largest { // std::cmp::PartiaOrd
            largest = item;
        }
    }
    largest
}

fn main() {
    let str_list = vec![String::from("hello"), String::from("world")];
    let result = largest(&str_list);
    println!("The largest word is {}", result);
}
