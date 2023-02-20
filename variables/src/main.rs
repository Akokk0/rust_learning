fn main() {
    // 循环 loop while for

    // loop
    let mut a = 0;
    let result = loop { // loop会一直执行，直到使用break喊停为止
        // println!("again!---{}", a);

        if a == 10 {
            break a * 2;
        }

        a += 1
    };

    println!("The value of result is: {}", result);

    // while 效率相对较低，因为每次循环前需要做判断
    let mut number = 3;

    while number != 0 {
        // println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // while 在这样的场景下容易出错，容易造成越界的问题
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        // println!("the value is: {}", a[index]);
        index += 1;
    }

    // 使用 for 可以解决这样的问题
    for _element in a.iter() {
        // println!("The value is: {}", element);
    }

    // 新版本可简写为
    for _element in a {
        // println!("The value is {}", element);
    }

    // Range(由标准库提供) 写法：(1..4) 可取1到3，左闭右开
    for number in (1..4).rev() {
        println!("The value is {}", number);
    }
    println!("LIFTOFF");

}