fn main() {
    let x: (String, i32, &str) = (String::from("Hello"), 2, "Hello");

    println!("{}", x.0);

    // Define a mutable tuple with a Vec as one of the elements
    let mut my_tuple: (String, Vec<i32>) = (String::from("Numbers"), vec![1, 2, 3]);

    // Access and print the tuple elements
    println!("Label: {}", my_tuple.0); // "Numbers"
    println!("Values: {:?}", my_tuple.1); // [1, 2, 3]

    // Modify the Vec (variable-length data)
    my_tuple.1.push(4);
    my_tuple.1.push(5);

    // Print the updated Vec
    println!("Updated Values: {:?}", my_tuple.1); // [1, 2, 3, 4, 5]
}
