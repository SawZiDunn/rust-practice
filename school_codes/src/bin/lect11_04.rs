fn main() {
    let mut text = format!("First ");
    let inp = std::io::stdin();
    inp.read_line(&mut text).unwrap();
    text.push_str("Second: ");
    inp.read_line(&mut text).unwrap();
    text.push_str("Third: ");
    inp.read_line(&mut text).unwrap();
    println!("{}: {}", text, text.len());
}
