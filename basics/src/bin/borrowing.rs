fn own_string(b: &String) {
    println!("c: {}", b);
}

fn own_vector(mut c: &Vec<i32>) -> Vec<i32> {
    let mut new_vector = Vec::new();
    new_vector.push(7);
    new_vector
}

fn main() {
    let mut y = String::from("Hello World");
    let mut z = vec![1, 2, 3, 4, 5];

    // this is borrowing by reference, so y is still valid after this point
    own_string(&y);
    println!("y: {}", y); // y is still accessible at this point, the scope ends here

    let mut x = &y; // can be borrowed with mutable again because the previous borrowing scope is over
    println!("x: {}", x);

    let new_vec = own_vector(&z);
    println!("new_vec: {:?}", new_vec);
}
