fn main() {
    let mut b = Box::new(5);
    *b += 5;
    println!("b = {}", b);
}
