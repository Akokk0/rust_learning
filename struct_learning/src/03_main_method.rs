/* struct的方法
        · 方法和函数类似：fn关键字、名称、参数、返回值
        · 方法与函数不同之处：
            - 方法是在struct(或enum、trait对象)的上下文中定义
            - 第一个参数是self，表示方法被调用的struct实例
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

/* 定义方法
    · 在impl块里定义方法
    · 方法的第一个参数可以是&self，也可以获得其所有权或可变借用。和其他参数一样
    · 更良好的代码组织
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

// 一个struct可以拥有多个impl块
impl Rectangle {
    /* 关联函数
        · 可以在impl块里定义不把self作为第一个参数的函数，它们叫关联函数(不是方法)
            - 例如：String::from()
        · 关联函数通常用于构造器
        · ::符号
            - 关联函数
            - 模版创建的命名空间
    */

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size
        }
    }
}

fn main() {
    // 使用关联函数当构造器
    let s = Rectangle::square(20);

    let rect = Rectangle {
        width: 30,
        length: 50
    };

    let rect2 = Rectangle {
        width: 40,
        length: 60
    };

    let rect3 = Rectangle {
        width: 30,
        length: 50
    };

    /* 方法调用的运算符
        · C/C++：object -> something() 和 (*object).something() 一样
        · Rust没有 -> 运算符
        · Rust会自动引用或解引用
            - 在调用方法时就会发生这种行为
        · 在调用方法时，Rust根据情况自动添加&、&mut或*，以便object可以匹配方法的签名
        · 下面两行代码效果相同：
            - p1.distance(&p2);
            - (&p1).distance(&p2);
    */
    println!("{}", rect.area());
    println!("{}", (&rect).area()); // fn area(self) -> u32 如果方法要求move这样写就会报错，因为(&rect)是借用

    println!("{}", rect2.can_hold(&rect3));
}