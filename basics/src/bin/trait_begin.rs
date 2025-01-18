// Trait definition
trait QuarticRoot {
    fn quartic_root(self) -> Self;
}

// Implementing the QuarticRoot trait for f32
impl QuarticRoot for f32 {
    fn quartic_root(self) -> f32 {
        self.sqrt().sqrt()
    }
}

// Implementing the QuarticRoot trait for f64
impl QuarticRoot for f64 {
    fn quartic_root(self) -> f64 {
        self.sqrt().sqrt()
    }
}

// Function-style implementation
fn quartic_root<Banana: QuarticRoot>(x: Banana) -> Banana {
    // genereic function that work on type T that implements the QuartticRoot.
    x.quartic_root()
}

fn main() {
    // Function-style usage
    println!("{}", quartic_root(100f32)); // This works
    println!("{}", quartic_root(100f64)); // This works
}
