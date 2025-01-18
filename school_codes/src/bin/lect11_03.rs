fn main() {
    let mut line = String::new();
    println!("Enter something!");

    match std::io::stdin().read_line(&mut line) {
        Ok(bytes_read) => {
            println!("Read {} bytes", bytes_read);
            println!("[{}]", line);
        }
        Err(error) => {
            println!("Error reading line: {}", error);
        }
    }
}
