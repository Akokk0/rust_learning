fn main() {
    // Some给了值可以直接推断
    let some_number = Some(5);
    let some_string = Some("A String");
    // None无法直接推断，必须添加泛型
    let absent_number: Option<i32> = None;

    // Some<i8> 与 i8不是同一类型
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; // 这里会报错 cannot add `Option<i8>` to `i8`
}