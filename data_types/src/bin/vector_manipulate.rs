use core::num;

fn get_item(index: usize) {
    //let index = 3; // this looks like an unsigned integer, but it's actually a usize
    // vector values can be accessed only with usize type
    let vec = vec![1, 2, 3, 4, 5];

    // Retrieve a value at a specific index
    // unwrap() is for changing option type to int
    let value = vec.get(index).unwrap();

    // print the value
    println!("The value at index {} is {:?}", index, value);
}

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    // for i in &numbers {
    //     println!("A reference to {}", i); // we can either put a star or not due to rust's auto dereferencing
    // }

    // for i in &mut numbers {
    //     println!("A mutable reference to {}", i);
    // }

    // for i in numbers.iter() {
    //     println!("{}", i); // // we can either put a star or not due to rust's auto dereferencing
    // }

    // take the ownership of numbers,
    // numbers cannot be used after this loop
    // for i in numbers {
    //     println!("{}", i);
    // }
    // println!("{:?}", numbers); // this will throw error cuz numbers is no longer valid at this point

    //getting the last value
    let last_item = numbers.last().unwrap();
    println!("{}", last_item);

    //getting the first value
    // match numbers.first() {
    //     Some(first_item) => println!("the first item: {}", first_item),
    //     None => println!("The vector is empty"),
    // }

    println!("{}", numbers.first().unwrap());

    // creating an empty vector with the type &str
    let mut fruits: Vec<&str> = vec![];
    let mut fruits: Vec<&str> = Vec::new();

    fruits.push("Apple");
    fruits.push("Orange");
    println!("{:?}", fruits);
    println!("Removed value: {:?}", fruits.pop());
    println!("{:?}", fruits);

    get_item(2);
}
