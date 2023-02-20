fn main() {
    // 复合类型，可以将多个值放在一个类型里
    // Rust 提供了两种基础的复合类型：元组(Tuple)、数组

    // 元组(Tuple)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 使用点标记法获取元素值
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    // 还可以使用模式匹配解构(destructure)一个Tuple来获取元素的值
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // 数组
    // 如果想让数据存放在stack(栈)上而不是head(堆)上，或者想保证有固定数量的元素
    let _arr = [1, 2, 3, 4, 5];

    // 数组没有 Vector灵活，和数组类似，由标准库提供，Vector的长度可以改变
    // 数组一个例子
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    // 数组的类型 表示：[类型; 长度]
    // 例如：let a: [i32; 5] = [1, 2, 3, 4, 5]
    // 另一种声明数组的方法
    let _const_arr = [3; 5]; // 相当于：let a = [3, 3, 3, 3, 3];

    // 数组元素的访问
    let a = months[0];
    println!("{}", a);

    // let index = [12, 13, 14, 15];
    // let month = months[index[1]]; // 编译可以通过，运行时报错
}