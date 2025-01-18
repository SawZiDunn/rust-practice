// Changing a struct
// Define a struct with mutable fields
struct DataHolder {
    name: String,
    values: Vec<i32>,
}

impl DataHolder {
    // Method to create a new instance of DataHolder
    fn new(name: &str) -> Self {
        DataHolder {
            name: name.to_string(),
            values: Vec::new(),
        }
    }

    // Method to add a value to the `values` Vec
    fn add_value(&mut self, value: i32) {
        self.values.push(value);
    }

    // Method to update the `name` field
    fn update_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    // Method to display the contents
    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Values: {:?}", self.values);
    }
}

fn main() {
    // Create a new DataHolder instance
    let mut holder = DataHolder::new("Initial");

    // Display initial contents
    holder.display();

    // Add values to the `values` Vec
    holder.add_value(1);
    holder.add_value(2);
    holder.add_value(3);

    // Display contents after adding values
    holder.display();

    // Update the `name` field
    holder.update_name("Updated Name");

    // Display contents after updating name
    holder.display();

    // Add more values to the `values` Vec
    holder.add_value(4);
    holder.add_value(5);

    // Display final contents
    holder.display();
}
