/* Struct，结构体
        - 自定义的数据类型
        - 为相关联的值命名，打包 => 有意义的组合
*/
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

/* Tuple struct
    - Tuple struct整体有个名，但里面的元素没有名
    - 适用：想给整个tuple起名，并让它不同于其他tuple，而且又不需要给每个元素起名
*/
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit-Like Struct
    - 可以定义没有任何字段的struct，叫做Unit-Like struct(因为与()，单元类型类似)
    - 适用于需要在某个类型上实现某个trait，但是在里面又没有想要存储的数据
*/

/* struct数据的所有权
    · 这里的字段使用了String而不是&str：
        - 该struct实例拥有其所有的数据
        - 只要struct实例是有效的，那么里面的字段数据也是有效的
    · struct里也可以存放引用，但这需要使用生命周期
        - 生命周期保证只要struct实例是有效的，那么里面的引用也是有效的
        - 如果struct里面存储引用，而不使用生命周期，就会报错
*/

fn main() {
    // 不能缺少字段
    let user1 = User {
        email: String::from("admin@akokko.com"),
        username: String::from("Akokko"),
        active: true,
        sign_in_count: 556
    };

    // 一旦struct的实例是可变的，那么实例中所有字段都是可变的
    let mut user2 = User {
        email: String::from("admin@akokko.com"),
        username: String::from("Akokko"),
        active: true,
        sign_in_count: 556
    };

    user2.email = String::from("admin@akokko.moe");

    // Struct 更新语法
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // 可以把另外两项展开
    };

    // Tuple Struct  black和origin是不同的类型，是不同tuple struct的实例
    let black = Color(0, 0, 0);
    let origin = Point(0, 0 , 0);
}

// Struct 作为函数的返回值
fn build_user(email: String, username: String) -> User {
    User {
        // 字段初始化简写
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
