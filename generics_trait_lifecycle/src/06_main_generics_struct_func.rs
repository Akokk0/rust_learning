struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 如果针对特定类型就不需要加<T>
impl Point<i32> {
    // 并且只有该类型才有该方法
    fn x1(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let p = Point {x: 5, y: 10};
    println!("p.x = {}", p.x());
}