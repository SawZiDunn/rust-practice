// Closure
fn main() {
    let factor = 2;
    let multiply = |x| x * factor; // Closure capturing `factor` from its environment
    println!("factor = {}", factor);

    let result = multiply(5); // `x` is 5, `factor` is captured by reference
    println!("The result is: {}", result); // Output: "The result is: 10"

    let factor = 3; // Even if we change `factor` here, it does not affect the closure because it captures the earlier environment context where `factor` was 2.
    let new_result = multiply(5); // Still uses the old `factor` value of 2
    println!("factor = {}", factor); // factor is 3 now but
    println!("The new result is: {}", new_result); // Output: "The new result is: 10" (not 15)

    let divide: fn(i32, i32) -> i32 = |a, b| a / b;
    println!("divide {}", divide(8, 3));
}
