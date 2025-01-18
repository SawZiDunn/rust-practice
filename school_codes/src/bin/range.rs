use core::num;

fn main() {
    // Open-ended ranges
    let numbers = [1, 2, 3, 4, 5];

    // array slice
    let my_range = ..3;
    // let up_to_three = &numbers[..3];
    let up_to_three = &numbers[my_range];
    println!("Up to 3: {:?}", up_to_three); // [1, 2, 3]

    // RangeFull: ..
    let all_numbers = &numbers[..];
    println!("All numbers: {:?}", all_numbers); // [1, 2, 3, 4, 5]

    // Inclusive range (inclusive on the upper bound)
    // RangeToInclusive: ..=3
    let up_to_three_inclusive = &numbers[..=3];
    println!("Up to and including 3: {:?}", up_to_three_inclusive); // [1, 2, 3]

    // Demonstrating type inference with ranges
    let inferred_range = 0..5; // Range<i32>
    let inferred_range2 = 0u8..5; // Range<i32>
    let explicit_range: std::ops::Range<u32> = 0..5;

    println!("Inferred range: {:?}", inferred_range);
    println!("Inferred range2: {:?}", inferred_range2);
    println!("Explicit range: {:?}", explicit_range);

    println!(
        "Type of inferred_range: {:?}",
        std::any::type_name_of_val(&inferred_range)
    );
    println!(
        "Type of explicit_range: {:?}",
        std::any::type_name_of_val(&inferred_range2)
    );
    println!(
        "Type of explicit_range: {:?}",
        std::any::type_name_of_val(&explicit_range)
    );
}
