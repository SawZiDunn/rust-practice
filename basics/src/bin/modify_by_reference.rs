fn modify_by_value(mut x: i32) {
    x += 10;
    println!("Inside function: {}", x);
}
fn modify_by_reference(x: &mut i32) {
    *x += 10; // changing the original value of x
    println!("Inside function: {} {} {} {:p}", *x, x, &x, x as *const i32);
}

fn main() {
    let mut x = 20;
    modify_by_value(x);
    println!("In main: {}", x); // still 20
    modify_by_reference(&mut x); // passing mutable reference
    println!("In main: {}", x); // becomes 30
}
