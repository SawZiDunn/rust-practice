fn main() {
    // initializing all values to 0 with the length 5
    let bytes = [0; 5];

    // printing array
    let mut arr: [i32; 5] = [1, 2, 3, -4, 5];
    println!("{:?}", arr);
    let mut int = String::new();

    slice(&arr);

    // borrowing array to the function as a parameter
    new(&arr);
    println!("Enter a value: ");
    std::io::stdin()
        .read_line(&mut int)
        .expect("Failed to read a value");

    arr[0] = int.trim().parse().expect("Invalid Integer!");

    for i in arr.iter() {
        println!("{}", i);
    }
}

fn new(nums: &[i32]) {
    let sum: i32 = nums.iter().sum();
    println!("{sum}");
}

fn slice(nums: &[i32]) {
    for (index, i) in nums.iter().enumerate() {
        if *i < 0 {
            // * can be omitted
            panic!("Negative number found at index {}", index);
        }
    }
}

fn slicing() {
    let mut x = [1, 2, 3, 4, 5];
    let y = &x[0..3];

    println!("x: {:?}", x);
    println!("y: {:?}", y);
}
