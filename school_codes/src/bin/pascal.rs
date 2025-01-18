use std::io;

fn pascal(row: usize, col: usize) -> usize {
    if col == 0 {
        1
    } else if col == row {
        1
    } else {
        pascal(row - 1, col - 1) + pascal(row - 1, col)
    }
}

fn print_pascal_row(n: usize, row: usize) {}

fn main() {
    let n: usize = loop {
        println!("Enter a number between 1 and 9: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(num) if (1..9).contains(&num) => break num,
            _ => println!("Please enter a valid number between 1 and 9."),
        }
    };

    let mut space = (n + n - 1) / 2;

    for row in 0..n {
        for _space in 0..space {
            print!("");
        }

        for j in 0..row + 1 {
            print!("{} ", pascal(row, j));
        }

        for _space in 0..space {
            print!("");
        }
        println!("");

        space -= 1;
    }
}
