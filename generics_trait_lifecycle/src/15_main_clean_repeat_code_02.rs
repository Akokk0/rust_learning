fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    // 模式匹配
    for &item in list {
        if item > largest { // std::cmp::PartiaOrd
            largest = item;
        }
    }
    largest
}

fn main() {
    // 重复代码
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
