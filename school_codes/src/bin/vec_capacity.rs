fn main() {
    let mut v = Vec::new();
    for i in 0..80 {
        v.push(i);
        println!(
            "Length: {}, Capacity: {}, Ref {:p}",
            v.len(),
            v.capacity(),
            &v
        );
    }
}
