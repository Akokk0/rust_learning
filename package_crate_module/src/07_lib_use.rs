mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn some_function() {}
    }
}

use crate::front_of_house::hosting; // 绝对路径
// use front_of_house::hosting; // 相对路径

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // hosting::some_function(); // 私有函数仍不可以访问
}