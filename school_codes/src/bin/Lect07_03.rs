use std::ptr;

fn main() {
    let n = [20, 11, 29];
    let mut n_ref = &n[1];
    println!("{:p} {}", n_ref, n_ref);
    let n_ref2 = n_ref + 2;
    println!("{}", n_ref2);
    println!("{:?}", n);
}
