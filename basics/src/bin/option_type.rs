fn divide(x: i16, y: i16) -> Option<i16> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

fn safe_sqrt(number: f64) -> Result<f64, String> {
    if number < 0.0 {
        Err("Cannot take the square root of a negative number.".to_string())
    } else {
        Ok(number.sqrt())
    }
}

fn main() {
    let age: Option<u8> = Some(25);
    match age {
        Some(x) => println!("Age is {}", x),
        None => println!("No age provided"),
    }

    let ans = divide(25, 0);
    println!("{:?}", ans);
    println!("{}", ans.unwrap_or(0)); // ans.unwrap_or(0) default value 0

    match ans {
        Some(num) => println!("Ans: {}", num),
        None => println!("Cannot divide"),
    }

    if ans.is_some() {
        println!("Ans: {}", ans.unwrap());
    } else {
        println!("Cannot divide!");
    }

    let number = -10.0;
    match safe_sqrt(number) {
        Ok(sqrt) => println!("The square root of {} is {}", number, sqrt),
        Err(err) => println!("Error: {}", err),
    }
}
