// Define trait A
trait ATrait {
    fn do_something_a(&self);
}

// Implement trait A for f32
impl ATrait for f32 {
    fn do_something_a(&self) {
        println!("Trait A implemented for f32");
    }
}

// Implement trait A for f64
impl ATrait for f64 {
    fn do_something_a(&self) {
        println!("Trait A implemented for f64");
    }
}


// Define trait B
trait BTrait {
    fn do_something_b(&self);
}


// Implement trait B for f32
impl BTrait for f32 {
    fn do_something_b(&self) {
        println!("Trait B implemented for f32");
    }
}

// Implement trait B for f64
impl BTrait for f64 {
    fn do_something_b(&self) {
        println!("Trait B implemented for f64");
    }
}

// Define a generic function that requires both traits A and B
fn example<T>(x: T)
 where                       // where clause with multiple trait bounds.
    T: ATrait + BTrait,     // T must implement both ATrait and BTrait
{
    x.do_something_a();
    x.do_something_b();
}

fn main() {
    let value_f32: f32 = 42.0;
    let value_f64: f64 = 42.0;

    // Call the generic function with f32 and f64
    example(value_f32); // This works because f32 implements both A and B
    example(value_f64); // This works because f64 implements both A and B
}
