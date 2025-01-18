// Define a trait for describing an item
trait Describable {
    fn describe(&self) -> String;
}

// Implement the trait for a struct "Book"
struct Book {
    title: String,
    author: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("'{}' by {}", self.title, self.author)
    }
}

// Implement the trait for another struct "Car"
struct Car {
    make: String,
    model: String,
}

impl Describable for Car {
    fn describe(&self) -> String {
        format!("{} {}", self.make, self.model)
    }
}

// A generic function that works with any type that implements the Describable trait
fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn main() {
    let book = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
    };
    let car = Car {
        make: String::from("Toyota"),
        model: String::from("Corolla"),
    };

    print_description(&book); // Output: '1984' by George Orwell
    print_description(&car); // Output: Toyota Corolla
}
