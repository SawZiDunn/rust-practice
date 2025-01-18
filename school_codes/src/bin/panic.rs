fn min(arr: &[i32]) -> i32 {
    // Handle empty slices
    if arr.is_empty() {
        panic!("Cannot find minimum of an empty slice");
    }

    let mut minimum = arr[0]; 

    for &num in arr.iter().skip(1) { 
        if num < minimum {
            minimum = num;
        }
    }

    minimum
}

fn main() {
    let numbers = [5, 2, 9, 1, 7];
    let minimum_value = min(&numbers);
    println!("The minimum value is: {}", minimum_value);
}