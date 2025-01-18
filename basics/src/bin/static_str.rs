use std::mem::*;

fn main() {
    let a: &str = "";
    let b: &str = "0123456789";
    let c: &str = "abcdè";
    println!("{} {} {}; ", size_of_val(a), size_of_val(b), size_of_val(c));

    println!(
        "{} {} {}; ",
        size_of_val(&a), //&&str
        size_of_val(&b),
        size_of_val(&c)
    );

    println!(
        "{} {} {}; ",
        size_of_val(&&a),
        size_of_val(&&b),
        size_of_val(&&c)
    );

    println!("{}", size_of::<&str>());
}
