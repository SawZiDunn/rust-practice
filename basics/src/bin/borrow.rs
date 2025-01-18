// Borrow
fn main() {
    let x = 5; // x owns the integer
    let y = x; // x is copied, not moved
    println!("x = {}, y = {}", x, y); // Both are accessible

    let s1 = String::from("hello"); // s1 owns the string
    let s2 = s1; // ownership moves to s2
                 // println!("{}", s1);                  // This would cause a compile error
    println!("{}", s2); // This is fine

    let mut s = String::from("hello");

    // Immutable borrow
    let r1 = &s; // OK
    let r2 = &s; // OK - multiple immutable borrows are allowed
    println!("{} and {}", r1, r2);

    // Immutable borrow for calculating length
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);

    // Mutable borrow to change the string
    change(&mut s);
    println!("After change, s is: {}", s);

    // Mutable borrow (only allowed because r1 and r2 are no longer used)
    let r3 = &mut s;
    r3.push_str("!");
    // let r4 = &mut s; // will throw error since s is already borrowed mutably once
    println!("After mutable borrow, s is: {}", s); // s and r3 are the same
}

fn calculate_length(s: &String) -> usize {
    s.len() // s is borrowed immutably
}

fn change(s: &mut String) {
    s.push_str(", world"); // s is borrowed mutably
}
