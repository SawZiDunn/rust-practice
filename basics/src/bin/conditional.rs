fn main() {
    conditional_2();
}

fn conditional() {
    let speed = 122;
    let min_speed = 40;
    let max_speed = 120;

    if speed < 40 {
        println!(
            "you are driving below the speed limit by {} km/hr",
            min_speed - speed,
        );
    } else if speed < 120 && speed > 40 {
        println!("you are driving within the speed limit");
    } else if speed > 120 {
        println!(
            "you are driving above the speed limit by {} km/hr",
            speed - max_speed
        );
    }
}

fn conditional_1() {
    let number: i32 = 0;
    if number > 0 && number % 2 != 0 {
        println!("number {} is a positive odd number", number)
    } else if number >= 0 && number % 2 == 0 {
        println!("number {} is a positive even number", number);
    } else if number < 0 && number % 2 != 0 {
        println!("number {} is a negative odd number", number);
    } else if number < 0 && number % 2 == 0 {
        println!("number {} is a negative even number", number);
    };
}

fn conditional_2() {
    let x: bool = false;

    let y = if x { 32 } else { 46 };

    println!("{y}");
}
