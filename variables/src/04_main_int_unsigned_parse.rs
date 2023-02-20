fn main() {
    // 在 parse() 这样的方法中，转换可能产生很多值，所以必须要指明接收变量的类型
    let guess: u32 = "42".parse().expect("Not a number");

    println!("{}", guess);

    /*
        int 包含 i8 i16 i32 i64 i128 分别对应int的多少位
        unsigned int包含 u8 u16 u32 u64 u128 分别对应unsigned int的多少位
        int 数据类型默认为 i32
        补充：isize和usize对应计算机位数的有符号和无符号
        number表示：
            Decimal 十进制  98_222
            Hex 十六进制  0xff
            Octal 八进制  0o77
            Binary 二进制  0b1111_0000
            Byte(u8 only) 字节类型，只能使用u8  b'A'
    */
}