use std::io::{self, Write};

fn main() {
    let mut stdout = io::stdout();
    stdout.write(b"Hello, Rust!").unwrap();
    stdout.flush().unwrap(); // Ensure the output is flushed to the console
    stdout.write("Hello ".as_bytes()).unwrap();
    stdout.write(String::from("world!").as_bytes()).unwrap();
    stdout.flush().unwrap();
}
