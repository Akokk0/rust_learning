fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Crazy thursday, v me 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x)
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}
