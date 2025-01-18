use std::io;

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("Enter num: ");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: u32 = num.trim().parse().expect("Fail...");

    let ans = factorial(num);
    println!("The factorial of {} is {}.", num, ans);
}
