use std::fmt;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let point = Point { x: 0, y: 7 };
    // comes from Debug trait, allows you to print using {:?}
    println!("Point is {:?}", point);

    // comes from Display trait impl
    // means when point is printed using {}, it should display just like this
    // similar to def __str__() from python oop
    println!("Point is {}", point);
}
