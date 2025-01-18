fn main() {
    let maybe_number = Some(25);
    // let maybe_number: Option<Option<()>> = None;
    println!("{:?}", maybe_number);

    // checking if maybe_number is an option or not
    if let Some(number) = maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("There's no number.");
    }
}
