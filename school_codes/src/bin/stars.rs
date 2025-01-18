fn main() {
    let height: i32 = 9;
    println!("height: {}", height);
    for i in 0..height {
        for _j in 0..height - i - 1 {
            print!(" ");
        }
        for _k in 0..i + 1 {
            print!("*");
        }
        println!();
    }
}
