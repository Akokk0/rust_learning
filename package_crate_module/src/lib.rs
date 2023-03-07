// 当mod后面不是代码块而是;时，Rust会自动找src下名为该模块名的文件
mod front_of_house;

// 使用pub use相当于引入又导出，其他模块引入该模块也可以引入这个模块
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}