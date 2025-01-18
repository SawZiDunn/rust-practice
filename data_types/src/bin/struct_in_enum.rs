// Define a struct to represent the dimensions of a screen
struct Screen {
    width: u32,
    height: u32,
}

// Define an enum to represent different types of events
enum Event {
    Resize(Screen),
    Click(u32, u32),
}

// Function to handle events based on their type
fn handle_event(event: Event) {
    match event {
        Event::Resize(screen) => {
            println!("Screen resized to {}x{}", screen.width, screen.height);
        },
        Event::Click(x, y) => {
            println!("Click at coordinates ({}, {})", x, y);
        },
    }
}

fn main() {
    // Create instances of the Screen struct
    let screen_resize = Screen { width: 1920, height: 1080 };
    
    // Create instances of the Event enum
    let resize_event = Event::Resize(screen_resize);
    let click_event = Event::Click(100, 200);

    // Handle the events
    handle_event(resize_event);
    handle_event(click_event);
}
