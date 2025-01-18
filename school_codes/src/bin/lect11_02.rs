use std::env;

fn main() {
    match env::var("HOME") {
        Ok(value) => println!("The HOME environment variable is: {}", value),
        Err(e) => println!("Couldn't read HOME: {}", e),
    }
    match env::var("PWD") {
        Ok(value) => println!("The PATH environment variable is: {}", value),
        Err(e) => println!("Couldn't read PATH: {}", e),
    }
}
