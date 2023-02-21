fn main() {
    // 可变引用 mutable reference
    // 可变引用有一个重要的限制：在特定作用域内，对某一块数据，只能有一个可变的引用

    let mut s = String::from("Hello World");

    // let s1 = &mut s;
    // let s2 = &mut s; // cannot borrow `s` as mutable more than once at a time

    // println!("The length of '{}' is {}.", s1, s2);

    // 这样的好处是可以在编译时防止数据竞争
    /* 以下三种行为下会发生数据竞争：
        - 两个或多个指针同时访问同一个数据
        - 至少有一个指针用于写入数据
        - 没有使用任何机制来同步对数据的访问
    */

    // 可以通过创建新的作用域，来允许非同时的创建多个可变引用(例子)
    {
        let s3 = &mut s;
    }

    let s4 = &mut s;
    // 这样他们就不在同一个作用域了

    // 另外一个限制，不可以同时拥有一个可变引用和一个不变的引用，多个不变的引用是可以的
    let mut str = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let s5 = &mut s; // cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}, {}", r1, r2, s5);
}