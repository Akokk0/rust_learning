struct Point {
    x: i32,
    y: i32,
    z: i32
}

fn main() {
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, ..} => println!("x is {}", x)
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last)
        }
    }

    match numbers {
        (.., second, ..) => { // 编译器无法判断出是中间的哪个元素
            println!("Some numbers: {}", second)
        }
    }
}