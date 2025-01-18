fn main() {
    //Converting a Slice (&[i32]) to a Vector (Vec<i32>)
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    let vector: Vec<i32> = slice.to_owned();  // Convert the slice to an owned Vec<i32>
    println!("{:?}", vector);  // Output: [1, 2, 3, 4, 5]

    //Converting a Slice of Tuples to an Owned Vec of Tuples
    let tuple_slice: &[(i32, i32)] = &[(1, 2), (3, 4)];
    let owned_tuple_vec: Vec<(i32, i32)> = tuple_slice.to_owned();  // Convert to Vec<(i32, i32)>
    println!("{:?}", owned_tuple_vec);  // Output: [(1, 2), (3, 4)]

}