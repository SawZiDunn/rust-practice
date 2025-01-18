fn main() {
    let mut a: String = "Xyz".to_string();
    a.remove(0); // remove the char at index 0 => "yz"
    println!("{}", a);
    a.insert(0, 'H'); //insert a char at index 0 => "Hyz"
    println!("{}", a);
    // pop removes the last char and returns an option contain some(n)
    let b: char = a.pop().expect(""); // "Hy"
    println!("{}  {}", a, b); //"Hy" "z"
    a.push('i'); // "Hyi"
    println!("{}", a);

    let s = String::from("Hello World - Rust");

    // change each splitted &str to String and store them in a vector
    let words: Vec<String> = s
        .split_whitespace()
        .map(|word| word.to_string()) // Convert &str to String
        .collect(); // Collect into a Vec<String>
    println!("{:?}", words); // Output: ["Hello", "World", "-", "Rust"]

    // just splitting only returns &str
    let s = "apple,banana,orange";
    let fruits: Vec<&str> = s.split(',').collect();
    println!("{:?}", fruits); // Output: ["apple", "banana", "orange"]

    let s = String::from("I love Rust");
    let s2 = s.replace("love", "like");
    println!("{}", s2); // Output: I like Rust

    let s = String::from("   Hello   Wold !    ");
    println!("{}", s.trim()); // Output: Hello   Wold !
}
