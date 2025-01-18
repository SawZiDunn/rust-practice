use std::fmt;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

// std::fmt::Display is for displaying any type of data
impl<T: fmt::Display, U> Point<T, U>
where
    U: fmt::Display,
{
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn display_point(&self) {
        // (self.x, self.y)
        println!("x: {}, y: {}", self.x, self.y);
    }

    fn mix_points<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point::new(25, 50);
    p1.display_point();

    let p2 = Point::new("hello", "world");
    p2.display_point();

    let p3 = Point::new(25, 5.5);
    p3.display_point();

    let p4 = Point::new('a', 'b');
    p4.display_point();

    println!("p4 and p3 mixed: {:?}", p4.mix_points(p3));
}
