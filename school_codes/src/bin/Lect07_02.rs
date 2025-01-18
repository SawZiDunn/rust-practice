// Static
use std::ptr;

static mut COUNTER: u32 = 0;

fn main() {
    let a: i32 = 10;
    println!("addr a: {:p} {}", &a, *(&a));
    println!(
        "Address and value: {:p} {}",
        get_address_and_increment(),
        get_and_increment()
    );
    println!(
        "Address and value: {:p} {}",
        get_address_and_increment(),
        get_and_increment()
    );
    println!(
        "Address and value: {:p} {}",
        get_address_and_increment(),
        get_and_increment()
    );
}

fn get_address_and_increment() -> *const u32 {
    unsafe {
        let addr = ptr::addr_of!(COUNTER);
        COUNTER += 1;
        addr
    }
}

fn get_and_increment() -> u32 {
    unsafe {
        let current_value = COUNTER;
        COUNTER += 1;
        current_value
    }
}
