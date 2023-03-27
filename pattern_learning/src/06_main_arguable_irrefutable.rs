fn main() {
    let a: Option<i32> = Some(5);
    // let Some(x) = a; // pattern `None` not covered

    // if let x = 5 {} // irrefutable `if let` pattern

    if let Some(x) = a {

    }
}
