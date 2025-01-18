use std;

fn try_print<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}

fn main() {
    // let mut x = Box::new(25);
    let x = 25;

    try_print(x);
    try_print(x); // we we used Box, this will throw error since x will be stored on heap and it's ownership is transferred
}
