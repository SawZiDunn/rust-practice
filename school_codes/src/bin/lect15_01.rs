trait Draw {
    fn draw(&self);
}

struct Circle;
impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

struct Square;

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
}

// Static dispatch - The type T implementing the Draw trait is determined at compile-time
fn draw_static<T: Draw>(item: T) {
    item.draw(); // Calls the appropriate draw() method at compile time
}

// Dynamic dispatch - The type implementing the Draw trait is determined at runtime
fn draw_dynamic(item: &dyn Draw) {
    item.draw(); // Calls the appropriate draw() method at runtime
}

fn main() {
    // Static dispatch example
    let circle = Circle;
    let square = Square;

    draw_static(circle); // Circle is known at compile time
    draw_static(square); // Square is known at compile time

    // Dynamic dispatch example
    let circle_dyn: &dyn Draw = &Circle;
    let square_dyn: &dyn Draw = &Square;
    draw_dynamic(circle_dyn); // Type checked at runtime
    draw_dynamic(square_dyn); // Type checked at runtime

    // dynamic dispatch also works this way, no need two lines like the above one
    // draw_dynamic(&circle);
}
