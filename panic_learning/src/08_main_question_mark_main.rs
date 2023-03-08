use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let result = File::open("hello.txt")?;
    Ok(())
}