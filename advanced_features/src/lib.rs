// macro_rules!声明宏(弃用)
// Rust中最常见的宏形式：声明宏

// let v: Vec<u32> = vec![1, 2, 3];

#[macro_export]
macro_rules! vec {
    ( $( $x: expr ), * ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}