use rand::{thread_rng, Rng};
use std::io::{self, Write}; // Import the rand crate's Rng trait

fn main() {
    // Generate a random number between 1 and 100
    let secret_number = thread_rng().gen_range(1..=100);

    // Initialize the boundaries of guesses
    let mut low_guess = 1;
    let mut high_guess = 100;

    loop {
        // Prompt the player for a guess
        print!("Enter your guess ({} < ? < {}): ", low_guess, high_guess);
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is displayed

        // Read the player's input
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        // Parse the input to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        // Check if the guess is correct
        if guess == secret_number {
            println!(
                "Congratulations! You guessed the right number: {}",
                secret_number
            );
            break;
        }

        // Update boundaries based on the guess
        if guess < secret_number {
            low_guess = guess + 1; // Update the lower boundary
            println!("{} < ?? < {}", low_guess - 1, high_guess); // Show the range
        } else {
            high_guess = guess - 1; // Update the upper boundary
            println!("{} < ?? < {}", low_guess, high_guess + 1); // Show the range
        }
    }
}
