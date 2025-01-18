fn get_first_char(text: &str) -> Option<char> {
    // Attempt to get the first character in the string
    let first_char = text.chars().next()?;
    // chars().next() method returns Option<char>. If the string is empty, it returns None. If it's not empty, it returns Some(char).
    // ? will unwrap Some(char) and assign it to first_char. If None, the function will return early with None.
    // Return the first character converted to uppercase
    Some(first_char.to_ascii_uppercase())
}

fn main() {
    let example_text = "";
    match get_first_char(example_text) {
        Some(c) => println!("The first character is: {}", c),
        None => println!("The input string was empty."),
    }
}
