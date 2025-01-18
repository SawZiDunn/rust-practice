use std::io;

fn main() {
    example_3();
}

fn example_1() {
    for i in 1..5 {
        println!("{}", i);
    }

    println!();

    for i in (1..=5).rev() {
        println!("{}", i);
    }

    println!();

    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers {
        println!("{}", n);
    }
}

fn example_2() {
    let mut n = 0;

    let x = loop {
        n += 1;
        println!("{n}");
        if n == 6 {
            break n;
        }
    };

    println!("Return Value: {x}");
}

fn example_3() {
    let mut input = String::new();
    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word! (Press 'stop' to exit!)");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");
    }
    println!("Good Bye!");
}
