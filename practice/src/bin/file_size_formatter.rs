use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

impl Sizes {
    fn format_return(size: f64) -> Self {
        Self {
            bytes: format!("{} bytes", size),
            kilobytes: format!("{:.2} kilobytes", size / 1000.),
            megabytes: format!("{:.2} megabytes", size / 1_000_000.),
            gigabytes: format!("{:.2} gigabytes", size / 1_000_000_000.),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    if args.len() != 2 {
        println!("Please provide an argument!");
        return;
    }

    let parts: Vec<&str> = args[1].split_whitespace().collect();
    // println!("Size: {}.", parts[0]);
    // println!("Unit: {}.", parts[1]);

    if parts.len() != 2 {
        println!("Please provide a format like '24 mb'! ");
        return;
    }

    let size: f64 = parts[0].trim().parse().expect("Failed to convert to f64!");
    let unit = parts[1].to_lowercase();

    let sizes: Sizes = match unit.as_str() {
        "b" | "bytes" => Sizes::format_return(size),
        "kb" | "kilobytes" => Sizes::format_return(size * 1000.),
        "mb" | "megabytes" => Sizes::format_return(size * 1_000_000.),
        "gb" | "gigabytes" => Sizes::format_return(size * 1_000_000_000.),
        _ => {
            println!("Invalid Unit!");
            return;
        }
    };

    println!("{:?}", sizes);
}
