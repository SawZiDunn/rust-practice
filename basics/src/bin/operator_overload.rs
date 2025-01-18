use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // Point and Self can be used interchangeably

    // the result of adding two Point objs will also be Point
    type Output = Point; // Self

    fn add(self, other: Self) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

//static dispatch
// function works for any type that implements Add
fn add_two_objs<T: Add<Output = T>>(p1: T, p2: T) -> T {
    p1 + p2
}

// dynamic dispatch is not suitable for this case cuz operator overloading requires actual types
// to be known at compile time, not at runtime
// in dynamic dispatch, types are known only at runtime

fn main() {
    let p1 = Point { x: 5, y: 3 };
    let p2 = Point { x: 2, y: 6 };

    let p3 = add_two_objs(p1, p2);
    println!("{:?}", p3);
}
