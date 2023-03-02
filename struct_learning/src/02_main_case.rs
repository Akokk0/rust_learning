// 要想为struct添加debug功能需要添加该宏
// 相当于让Rectangle派生于Debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

fn main() {
    let w = 30;
    let l = 50;

    let _rect = (w, l);

    let rect_example = Rectangle {
        width: 30,
        length: 50
    };

    // println!("{}", area(w, l));

    // 这里发生了借用，所有所有权还是在rect_example上，在main中还可以继续使用
    println!("{}", area_02(&rect_example));

    // 使用rect_example
    // println!("{}", rect_example); // the trait `std::fmt::Display` is not implemented for `Rectangle`

    // 调试Debug的格式化字符串
    // println!("{:?}", rect_example); // the trait `Debug` is not implemented for `Rectangle`

    // 添加Debug宏之后就可以打印了
    println!("{:?}", rect_example); // the trait `Debug` is not implemented for `Rectangle`
    // {}里多加入#会打印更加详细
    println!("{:#?}", rect_example); // the trait `Debug` is not implemented for `Rectangle`
}

// 计算长*宽，但是并不够好，传进来的长和宽没有关联
fn _area(width: u32, length: u32) -> u32 {
    width * length
}

// 改进
fn _area_01(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

// 通过struct改进
fn area_02(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
