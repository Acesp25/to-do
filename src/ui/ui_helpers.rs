use std::io::{self, Write};
use crate::planner::Planner;

pub fn display_menu() {
    println!("  Rust To-Do Planner!");
    println!("      1. Display today's plans");
    println!("      2. Create an event");
    println!("      3. Adjust an event");
    println!("      4. Display ALL events");
    println!("      5. Exit");
}

pub fn create_event() {
    // add implementation later
}

pub fn adjust_menu(planner: Planner) {
    println!("  Select an event to edit! (enter eventID)");
    planner.list_events(planner);


    if let Some(event) = planner.find_event_mut(event_id) {

    }
}

pub fn id_input() -> Option<usize>{
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error reading input: {}", e);
        return;
    }

    match input.trim().parse::<usize>() {
        Ok(id) => Some(id),
        Err(_) => {
            println!("Invalid event ID entered.");
            None
        }
    }
}
