/*fn returns_closure() -> Fn(i32) -> i32 {
    |x| x + 1
}*/

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {

}