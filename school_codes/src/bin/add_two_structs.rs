#[derive(Debug)] // Automatically implements the Debug trait for Point
struct Point {
    x: i32,
    y: i32,
}

// Implement Add for &Point (reference instead of moving)
impl std::ops::Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let result = &p1 + &p2; // Calls add method with references
    println!("{:?} + {:?} = {:?}", p1, p2, result);
}
