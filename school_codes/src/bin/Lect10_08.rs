fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // collect(): Gather elements into a collection
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Collect into a different collection type (HashSet)
    use std::collections::HashSet;
    let unique: HashSet<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("Unique doubled: {:?}", unique);

    // fold(): Reduce to a single value
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum using fold: {}", sum);

    // Custom fold: find the product of all even numbers
    let even_product = numbers.iter().fold(1, |acc, &x| {
        if x % 2 == 0 { acc * x } else { acc }
    });
    println!("Product of even numbers: {}", even_product);

    // sum(): Numeric reduction
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    // product(): Numeric reduction
    let product: i32 = numbers.iter().product();
    println!("Product: {}", product);

    // max(): Find maximum value
    if let Some(&max_val) = numbers.iter().max() {
        println!("Maximum: {}", max_val);
    }

    // min(): Find minimum value
    if let Some(&min_val) = numbers.iter().min() {
        println!("Minimum: {}", min_val);
    }

    // any(): Check if any element satisfies a condition
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("Has even number: {}", has_even);

    // all(): Check if all elements satisfy a condition
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("All positive: {}", all_positive);

    // Combining consumers: find the maximum even number
    let max_even = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .max();
    if let Some(&max_even_val) = max_even {
        println!("Maximum even number: {}", max_even_val);
    } else {
        println!("No even numbers found");
    }
}