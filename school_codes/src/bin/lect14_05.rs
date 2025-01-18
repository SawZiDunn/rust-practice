// Define an extension trait
trait HasLength {
    fn length(&self) -> usize;
    }

// Implement the extension trait for the external `String` length
impl HasLength for str {
    fn length(&self) -> usize { self.chars().count() }
}

// Define an extension trait
trait StringExtensions {
    fn is_alphabetic(&self) -> bool;
}

// Implement the extension trait for the external `String` type
impl StringExtensions for String {
    fn is_alphabetic(&self) -> bool {
        self.chars().all(|c| c.is_alphabetic())
    }
}

fn main() {
    let my_string = String::from("HelloWorld");
    let not_alphabetic_string = String::from("Hello123");

    println!("{} {}", my_string.len(), my_string.length());

    println!("Is alphabetic: {}", my_string.is_alphabetic()); // true
    println!("Is alphabetic: {}", not_alphabetic_string.is_alphabetic()); // false
}
