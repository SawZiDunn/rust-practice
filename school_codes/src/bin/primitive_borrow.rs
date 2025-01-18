fn main() {
    let mut a = 5;
    let b = &a; // immutable borrow
    println!("a: {}, b: {}", a, b);

    // cannot make mutable borrow here since immutable borrow is already made
    // it's either one mutable borrow or multiple immutable borrow
    // let d = &mut a;

    a += 10; // cannot assign a since it's already borrowed
    println!("a: {}, b: {}", a, b);
}
