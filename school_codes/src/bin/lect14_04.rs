// 1)  Define the base Traits
trait HasSquareRoot {
    fn sqrt(self) -> Self;
}

impl HasSquareRoot for f32 {
    fn sqrt(self) -> Self { self.sqrt() }
}

impl HasSquareRoot for f64 {
    fn sqrt(self) -> Self { self.sqrt() }
}

trait HasAbsoluteValue {
    fn abs(self) -> Self;
}

impl HasAbsoluteValue for f32 {
    fn abs(self) -> Self { self.abs() }
}

impl HasAbsoluteValue for f64 {
    fn abs(self) -> Self { self.abs() }
}

// 2) Define new Trait
trait HasSqrtAndAbs: HasSquareRoot + HasAbsoluteValue {
    fn double_abs_sqrt(self) -> Self; // Adding a new method
 }

// 3) Implement new Trait
impl HasSqrtAndAbs for f32 {
    fn double_abs_sqrt(self) -> Self {
        self.abs().sqrt() * 2.0 // An example method that calculates double the square root of abs
    }
}

impl HasSqrtAndAbs for f64 {
    fn double_abs_sqrt(self) -> Self {
        self.abs().sqrt() * 2.0
    }
}

// 4) Define a Generic Function Using the new Trait
fn abs_quartic_root<Number>(x: Number) -> Number
where
    Number: HasSqrtAndAbs,
{
    x.abs().sqrt().sqrt()
}

fn main() {
    let value_f32: f32 = -16.0;
    let value_f64: f64 = -16.0;

    // Using the generic function with f32 and f64
    println!("Quartic root of abs for f32: {}", abs_quartic_root(value_f32)); // Output: 2.0
    println!("Quartic root of abs for f64: {}", abs_quartic_root(value_f64)); // Output: 2.0

    // Using the new method defined in HasSqrtAndAbs
    println!("Double abs sqrt for f32: {}", value_f32.double_abs_sqrt()); // Output: 8.0
    println!("Double abs sqrt for f64: {}", value_f64.double_abs_sqrt()); // Output: 8.0
}