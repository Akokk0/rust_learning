/* 强大的控制流运算符 - match
    · 允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
    · 模式可以是字面值、变量名、通配符...
*/
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

/* 绑定值的模式
    · 匹配的分支可以绑定到被匹配对象的部分值
        - 因此，可以从enum变体中提取值
*/
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // 如果要执行多条语句可以使用{}包裹
        Coin::Penny => {
            println!("Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // 绑定值的模式匹配
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));
}