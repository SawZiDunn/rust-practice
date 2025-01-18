enum Result1<SuccessCode, FailureCode> {
    Success(SuccessCode),
    Failure(FailureCode, char),
    Uncertainty,
}

fn main() {
    // Create a mutable variable `res` of type `Result1<u32, u16>` and initialize it with `Success` variant
    let mut res = Result1::Success::<u32, u16>(12u32);

    // Print the initial state
    match &res {
        Result1::Success(val) => println!("Initial state: Success with value {}", val),
        Result1::Failure(code, c) => {
            println!("Initial state: Failure with code {} and char {}", code, c)
        }
        Result1::Uncertainty => println!("Initial state: Uncertainty"),
    }

    // Reassign `res` to the `Uncertainty` variant
    res = Result1::Uncertainty;

    // Print the state after reassignment to `Uncertainty`
    match &res {
        Result1::Success(val) => println!("State after reassignment: Success with value {}", val),
        Result1::Failure(code, c) => println!(
            "State after reassignment: Failure with code {} and char {}",
            code, c
        ),
        Result1::Uncertainty => println!("State after reassignment: Uncertainty"),
    }

    // Reassign `res` to the `Failure` variant with a `u16` value and a `char`
    res = Result1::Failure(0u16, 'd');

    // Print the state after reassignment to `Failure`
    match &res {
        Result1::Success(val) => println!("State after reassignment: Success with value {}", val),
        Result1::Failure(code, c) => println!(
            "State after reassignment: Failure with code {} and char {}",
            code, c
        ),
        Result1::Uncertainty => println!("State after reassignment: Uncertainty"),
    }
}
