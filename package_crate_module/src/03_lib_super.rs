fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // 相对路径
        crate::serve_order(); // 绝对路径
    }

    fn cook_order() {}
}