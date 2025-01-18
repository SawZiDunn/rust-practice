fn ownership() {
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[2..];
    println!("{:?}", slice);
}

fn modifiable() {
    //declaring and initializing vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[..]; // creates a slice of all elements in numbers
    slice[0] = 10;

    // this will throw an error since numbers is already borrowed once
    // let another_slice = &numbers[..];
    println!("{:?}", slice);
}

fn main() {
    // ownership();
    modifiable();

    // declaring initializing vector with the value 5 for 3 places
    let nums = vec![5; 3];
    println!("{:?}", nums);
}

// Use slices when:
// - you want to borrow a portion of a collection rather than the whole collection
// - you want to pass around a reference to a sequence of items without copying them
// - you want to access a subset of a collection without copying
// Use vectors when:
// - you need to dynamically grow or shrink your collection
// - you need to own the collection and transfer ownership to another scope
