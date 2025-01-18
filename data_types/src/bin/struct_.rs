use std::vec;

#[derive(Debug)]
struct Person {
    name: String,
    age: Option<u8>,
    hobby: Vec<String>,
}

#[derive(Debug)]
// Tuple struct
// Define a tuple struct named `Point`
struct Point(f64, f64);

impl Point {
    // Method to calculate the distance from the origin
    fn distance_from_origin(&self) -> f64 {
        ((self.0 * self.0) + (self.1 * self.1)).sqrt()
    }

    // Associated function to create a new `Point`
    fn new(x: f64, y: f64) -> Self {
        Point(x, y)
    }
}

fn main() {
    let mut john = Person {
        name: "John Doe".to_string(),
        age: Some(35), // or None
        hobby: vec!["Reading".to_string(), "Football".to_string()],
    };

    println!("{:?}", john);
    let name = john.name;
    john.name = "Jack".to_string();

    println!("{} {} {:?}", john.name, john.age.unwrap(), john.hobby);
    println!("{}", name); // still John Doe

    let p = Point::new(3.0, 4.0);

    println!("x: {}", p.0);
    println!("y: {}", p.1);
    // Call the method to calculate the distance from the origin
    println!("Distance from origin: {}", p.distance_from_origin());
}
