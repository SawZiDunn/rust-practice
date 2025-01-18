// Struct
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Method to borrow `self` immutably
    fn greet(&self) -> String {
        format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        )
    }

    // Method to borrow `self` mutably
    fn have_birthday(&mut self) {
        self.age += 1;
    }

    // Associated function to create a new instance
    fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(), // name: name.to_string()
            age: age,
        }
    }
}

fn main() {
    // Create a new Person instance using the associated function
    let mut person = Person::new("Alice", 30);

    // Call the greet method
    println!("{}", person.greet());

    // Call the have_birthday method to increment the age
    person.have_birthday();

    // Call the greet method again to see the updated age
    println!("{}", person.greet());
}
