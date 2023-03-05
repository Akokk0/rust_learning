mod front_of_house { // 父mod
    mod hosting { // 子mod
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving { // 子mod
        fn take_order() {}
        fn server_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // 绝对路径
    front_of_house::hosting::add_to_waitlist(); // 相对路径
}