fn main() {
    let mut s1: String = String::from("Hello!");
    println!("s1 = {}", s1);

    // s1 is borrowed to s2
    let s2 = &mut s1;
    println!("s2 = {}", s2);

    s2.push_str(" World");
    println!("s2 = {}", s2);
    println!("s1 = {}", s1); // s1 cannot be borrowed again since it's already borrow as mutable

    s1.push_str(" abc");
    println!("s1 = {}", s1);

    println!("s2 = {}", s2); //this will throw an error

    println!("=====================");
}
