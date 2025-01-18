use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very Accurate"),
    );

    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );

    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    let book: &str = "Programming in Rust";
    println!("Review for \'{}\': {:?}", book, reviews.get(book));

    let ancient: &str = "Ancient Roman History";
    println!("{ancient} removed!");
    reviews.remove(ancient);

    println!("Review for {ancient}: {:?}", reviews.get(ancient));
}
