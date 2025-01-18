// Returning a closure from a function
fn create_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
 }
 
 fn main() {
    let multiplier = create_multiplier(10);
    println!("5 * 10 = {}", multiplier(5));
 }

