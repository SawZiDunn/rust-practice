#[derive(Debug, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        // -> Rectangle
        Self { width, height } // Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let mut rectangle = Rectangle::new(60, 50);
    println!("{:#?}", rectangle);

    let mut rectangle0 = Rectangle::new(70, 55);

    let mut rectangle1 = Rectangle::new(30, 20);

    let area = rectangle.area();
    println!("{}", area);

    println!("{}", rectangle.can_hold(&rectangle1));
    println!("{:?}", rectangle1);
}
