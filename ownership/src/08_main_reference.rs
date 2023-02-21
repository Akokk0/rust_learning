fn main() {
    // Reference &
    // &符号就表示引用：允许你引用某些值而不取得其所有权

    let s1 = String::from("hello");
    let len_s1 = calculate_length(&s1);

    let mut s2 = String::from("aikko");
    let len_s2 = calculate_length_mut(&mut s2);

    println!("The length of '{}' is {}", s1, len_s1);
    println!("The length of '{}' is {}", s2, len_s2);
}

fn calculate_length(s: &String) -> usize { // 以引用作为函数参数的行为称为借用 borrow
    // 不可以修改借用的东西
    // s.push_str(", world"); // Cannot borrow immutable local variable `s` as mutable
    s.len()
} // s在离开作用域时，仍然会失效，但是由于s是引用不用于该值所有权，所以并不会在离开作用域时销毁该值

fn calculate_length_mut(s: &mut String) -> usize { // 在变量为可变，且应用为可变时，可对引用进行修改
    s.push_str(",this value has been changed");
    s.len()
}