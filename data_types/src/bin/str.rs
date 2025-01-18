use std::io;

fn main() {
    let mut x = String::new();
    println!("Input something: ");
    io::stdin().read_line(&mut x).expect("Failed to read input");

    println!("x: {}", x.len());

    let y = x.trim();

    println!("y: {}", y.len());

    let z = x.trim().to_string();
    println!("z: {}", z.len());

    let mut z = String::from(y);
    println!("z: {}", z.len());
}
