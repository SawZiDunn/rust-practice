use std::io;

fn factorial(x: u32) -> u32 {
    if x == 0 {
        1
    } else {
        x * factorial(x - 1)
    }
}

fn binomial_coefficient(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

fn pascal_triangle(n: u32) {
    for i in 0..=n {
        // Print leading spaces for formatting
        for _ in 0..(n - i) {
            print!("  ");
        }

        // Print the binomial coefficients
        for j in 0..=i {
            print!("{:4} ", binomial_coefficient(i, j));
        }
        println!();
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter rows: ");
    io::stdin().read_line(&mut input).expect("Failed..");
    let rows: u32 = input.trim().parse().expect("Failed");
    input.clear();

    pascal_triangle(rows);
}
