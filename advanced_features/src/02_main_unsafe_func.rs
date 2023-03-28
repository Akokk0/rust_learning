unsafe fn dangerous() {}

fn main() {
    // dangerous(); // call to unsafe function

    unsafe {
        dangerous();
    }
}