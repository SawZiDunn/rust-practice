struct Animal {
    name: String,
}

trait Dog {
    fn run(&self) -> String;
    fn bark(&self);
}

impl Dog for Animal {
    fn run(&self) -> String {
        format!("{} is running", self.name)
    }

    fn bark(&self) {
        println!("{} is barking", self.name);
    }
}
fn main() {
    let john = Animal {
        name: "John".to_owned(),
    };

    john.bark();
    println!("{}", john.run());
}
