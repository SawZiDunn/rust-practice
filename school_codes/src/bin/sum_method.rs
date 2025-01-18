fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    let array = [1, 2, 3, 4, 5];
    let vector = vec![6, 7, 8, 9, 10];
    
    println!("Sum of array: {}", sum(&array));
    println!("Sum of vector: {}", sum(&vector));
    println!("Sum of slice: {}", sum(&array[1..4]));
}