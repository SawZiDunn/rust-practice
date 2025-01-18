use std::fs::File;
use std::io;
use std::io::Read;

fn main() -> Result<(), io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);
    Ok(())
}
