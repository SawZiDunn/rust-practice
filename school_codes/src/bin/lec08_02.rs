fn as_bytes<T>(o: &T) -> &[u8] {
    unsafe { std::slice::from_raw_parts(o as *const _ as *const u8, std::mem::size_of::<T>()) }
}

fn main() {
    println!("{:?}", as_bytes(&1i8)); // Viewing an i8 integer
    println!("{:?}", as_bytes(&2i16)); // Viewing an i16 integer
    println!("{:?}", as_bytes(&3i32)); // Viewing an i32 integer
    println!("{:?}", as_bytes(&'A')); // Viewing a char ('A')
    println!("{:?}", as_bytes(&true)); // Viewing a boolean (true)
}
