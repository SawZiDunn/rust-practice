use std::fs::File;
use std::io::{self, Read};

fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?; // Returns early if File::open fails
    let mut content = String::new();
    file.read_to_string(&mut content)?; // Returns early if read_to_string fails
    Ok(content) // Returns the content if everything succeeds
}

fn main() {
    match read_file_content("example.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
