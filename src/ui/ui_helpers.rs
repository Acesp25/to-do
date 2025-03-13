use std::io::{self, Write};
use crate::planner::Planner;
use crate::event::{Reoccurance, Priority};
use chrono::NaiveDateTime;

pub fn display_menu() {
    println!("  Rust To-Do Planner!");
    println!("      1. Display today's plans");
    println!("      2. Create an event");
    println!("      3. Adjust an event");
    println!("      4. Display ALL events");
    println!("      5. Exit");
}

pub fn adjust_menu() {
    println!("  What would you like to do with this event?");
    println!("      1. Change name");
    println!("      2. Change start time");
    println!("      3. Change end time");
    println!("      4. Change reoccurance");
    println!("      5. Change note");
    println!("      6. Delete event");
    println!("      7. Exit");
}

pub fn create_event() {
    // add implementation later
}

pub fn adjust_event(planner: Planner) {
    println!("  Select an event to edit! (enter eventID)");
    planner.list_events(planner);

    let event_id = id_input();

    if let Some(event) = planner.find_event_mut(event_id) {
        loop {
            adjust_menu_options();
            print!("> ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut choice = String::new();
            if io::stdin().read_line(&mut choice).is_err() {
                println!("Error reading input");
                continue;
            }
            match choice.trim().parse::<usize>() {
                Ok(1) => change_name(event),
                Ok(2) => change_start_time(event),
                Ok(3) => change_end_time(event),
                Ok(4) => change_reoccurance(event),
                Ok(5) => change_note(event),
                Ok(6) => {
                    // Here you could call a planner function to remove the event.
                    println!("Delete event selected (deletion not implemented here).");
                },
                Ok(7) => {
                    println!("Exiting adjust menu.");
                    break;
                },
                _ => println!("Invalid option, please try again."),
            }
        }
    } else {
        println!("Event id {} does not match any events stored", event_id);
        return;
    }
}

fn change_reoccurance(event: &mut Event) {
    println!("Enter new reoccurance option (None, Daily, Weekly, Monthly, Yearly, Fornite): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
          println!("Error reading input: {}", e);
          return;
    }
    match input.trim().to_lowercase().as_str() {
          "none" => {
               event.set_reoccurance(Reoccurance::None);
               println!("Reoccurance updated.");
          },
          "daily" => {
               event.set_reoccurance(Reoccurance::Daily);
               println!("Reoccurance updated.");
          },
          "weekly" => {
               event.set_reoccurance(Reoccurance::Weekly);
               println!("Reoccurance updated.");
          },
          "monthly" => {
               event.set_reoccurance(Reoccurance::Monthly);
               println!("Reoccurance updated.");
          },
          "yearly" => {
               event.set_reoccurance(Reoccurance::Yearly);
               println!("Reoccurance updated.");
          },
          "fornite" => {
               event.set_reoccurance(Reoccurance::Fornite);
               println!("Reoccurance updated.");
          },
          _ => println!("Unknown reoccurance option."),
    }
}

fn id_input() -> Option<usize>{
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
