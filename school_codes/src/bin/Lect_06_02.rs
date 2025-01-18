use std::fmt::{Debug, Display};

// Define a function that accepts a slice of any type that implements the Debug trait
fn print_items<T: Debug + Display>(items: &[T]) {
    for item in items {
        println!("{}", *item); // * is not necessary
    }
}

fn main() {
    // Create slices of different types that implement the Debug trait
    let int_items = vec![1, 2, 3, 4, 5];
    let str_items = vec!["apple", "banana", "cherry"];
    let float_items = vec![1.1, 2.2, 3.3];

    // Call the print_items function with different types of slices
    println!("Printing integer items:");
    print_items(&int_items);

    println!("\nPrinting string items:");
    print_items(&str_items);

    println!("\nPrinting float items:");
    print_items(&float_items);
}
