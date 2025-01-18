fn own_string(mut a: String) {
    a.push_str(" World");
    println!("a: {a}");
}

fn make_copy(mut x: i32) {
    x += 1;
    println!("{}", x);
}

fn main() {
    let x = String::from("Hello");
    println!("x: {x}");
    let y = x.clone();
    println!("y: {y}");

    // x is still valid at this point since its ownership is not moved, but copied to y
    println!("x: {x}");

    // ownership of y is transferred to the function
    own_string(x);
    // x is no longer valid at this point
    // println!("x: {x}"); // this will throw error

    let z = 25;
    make_copy(z); // z is still valid after this since int, float, and &str are copied, not moved
    println!("z: {}", z); // z is still 25
}

fn test() {
    let x = String::from("Hello World");
    let y = x.clone();
    println!("{:?}", y);
    println!("{:?}", x); // x is still valid at this point since it is cloned to y
}
