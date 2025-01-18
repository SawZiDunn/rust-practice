// define a tuple struct
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct KeyPress(String, char);

// define a classic C struct
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum WebEvent {
    WeLoad(bool),

    WeKeys(KeyPress),

    WeClick(MouseClick),
}

enum DiskType {
    SSD,
    HDD,
}

// enum can be used as a data type inside a struct
#[derive(Debug)]
enum DiskSize {
    KB(u32),
    MB(u32),
    GB(u32),
}

fn main() {
    let click = MouseClick { x: 100, y: 200 };

    let keys = KeyPress(String::from("Crtl+"), 'N');

    let we_load = WebEvent::WeLoad(true);
    let we_click = WebEvent::WeClick(click);
    let we_key = WebEvent::WeKeys(keys);

    println!(
        "WebEvent enum structure::\n{:#?}\n{:#?}\n{:#?}",
        we_load, we_click, we_key
    );

    // let disk_type = DiskType::SSD;
    // let disk_size = DiskSize::GB(128);

    // match disk_type {
    //     DiskType::HDD => println!("HDD"),
    //     DiskType::SSD => println!("SSD"),
    // }

    // println!("{:?}", disk_size);
}
