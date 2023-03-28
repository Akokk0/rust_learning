// extern创建接口，其他语言通过它们可以调用Rust函数
// fn前添加extern关键字，并指定ABI
// 还需添加#[no_mangle]注解：避免Rust在编译时改变它的名称
// 不需要unsafe
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

fn main() {}