use std::io::{self, Write};
use chrono::NaiveDateTime;

use crate::backend::classes::planner::Planner;
use crate::backend::enums::reoccurance::Reoccurance;
use crate::backend::enums::priority::Priority;
use crate::backend::classes::event::Event;

pub fn display_menu(current_date_time: NaiveDateTime) {
    println!("\n\n  Rust To-Do Planner! Current date-time: {}", current_date_time);
    println!("      1. Display today's plans");
    println!("      2. Create an event");
    println!("      3. Adjust an event");
    println!("      4. Display ALL events");
    println!("      5. Exit");
}

pub fn adjust_menu() {
    println!("\n  What would you like to do with this event?");
    println!("      1. Change name");
    println!("      2. Change start time");
    println!("      3. Change end time");
    println!("      4. Change reoccurance");
    println!("      5. Change note");
    println!("      6. Delete event");
    println!("      7. Exit event adjustment");
}

pub fn get_choice() -> Option<usize> {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error reading input: {}", e);
        return None;
    }
    match input.trim().parse::<usize>() {
        Ok(choice) => Some(choice),
        Err(_) => {
            println!("Invalid input.");
            None
        }
    }
}

fn id_input() -> Option<usize> {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Error reading input: {}", e);
        return None;
    }
    match input.trim().parse::<usize>() {
        Ok(id) => Some(id),
        Err(_) => {
            println!("Invalid event ID entered.");
            None
        }
    }
}

pub fn get_unix_timestamp_from_input() -> Option<NaiveDateTime> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Error reading input.");
        return None;
    }
    let input = input.trim();

    match NaiveDateTime::parse_from_str(input, "%m-%d-%Y %H:%M") {
        Ok(dt) => Some(dt),
        Err(e) => {
            println!("Failed to parse date/time: {}", e);
            None
        }
    }
}

pub fn event_creater(planner: &mut Planner) {
    // Get event name
    print!("Enter event name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut name = String::new();
    if io::stdin().read_line(&mut name).is_err() {
        println!("Error reading event name.");
        return;
    }
    let name = name.trim().to_string();

    // Get start time
    print!("Enter start time (MM-DD-YYYY HH:MM): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let start_time = match get_unix_timestamp_from_input() {
        Some(time) => time,
        None => {
            println!("Invalid start time input.");
            return;
        }
    };

    // Get end time
    print!("Enter end time (MM-DD-YYYY HH:MM): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let end_time = match get_unix_timestamp_from_input() {
        Some(time) => time,
        None => {
            println!("Invalid end time input.");
            return;
        }
    };

    // Get Priority
    print!("Enter event priority (High, Medium, Low): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut priority_input = String::new();
    if io::stdin().read_line(&mut priority_input).is_err() {
        println!("Error reading priority.");
        return;
    }
    let priority = match priority_input.trim().to_lowercase().as_str() {
        "high" => Priority::High,
        "medium" => Priority::Medium,
        "low" => Priority::Low,
        other => {
            println!("Unrecognized priority '{}', defaulting to Medium.", other);
            Priority::Medium
        }
    };

    // Get Reoccurance
    print!("Enter event reoccurance (None, Daily, Weekly, Monthly, Yearly, Fornite): ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut rec_input = String::new();
    if io::stdin().read_line(&mut rec_input).is_err() {
        println!("Error reading reoccurance.");
        return;
    }
    let reoccurance = match rec_input.trim().to_lowercase().as_str() {
        "none" => Reoccurance::None,
        "daily" => Reoccurance::Daily,
        "weekly" => Reoccurance::Weekly,
        "monthly" => Reoccurance::Monthly,
        "yearly" => Reoccurance::Yearly,
        "fornite" => Reoccurance::Fornite,
        other => {
            println!("Unrecognized reoccurance '{}', defaulting to None.", other);
            Reoccurance::None
        }
    };

    // Get note
    print!("Enter event note: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut note = String::new();
    if io::stdin().read_line(&mut note).is_err() {
        println!("Error reading note.");
        return;
    }
    let note = note.trim().to_string();

    let completed = false; // New events are not completed by default

    // Let the planner assign the id automatically, using its create_event method
    planner.create_event(name, start_time, end_time, priority, reoccurance, note, completed);
    println!("Event created successfully and added to the planner.");
}

pub fn adjust_event(planner: &mut Planner) {
    println!("Select an event to edit! (enter eventID)");
    planner.list_events();

    let event_id = match id_input() {
        Some(id) => id,
        None => return,
    };

    if let Some(event) = planner.find_event_mut(event_id) {
        loop {
            event.display();
            adjust_menu();
            match get_choice() {
                Some(1) => change_name(event),
                Some(2) => change_start_time(event),
                Some(3) => change_end_time(event),
                Some(4) => change_reoccurance(event),
                Some(5) => change_note(event),
                Some(6) => {
                    delete_event(planner, event_id);
                    break;
                },
                Some(7) => {
                    println!("Exiting adjust menu.");
                    break;
                },
                _ => println!("Invalid option, please try again."),
            }
        }

        // Save changes after mutable borrow ends
        if let Err(e) = planner.save_events_to_file() {
            println!("Failed to save changes: {}", e);
        }
    } else {
        println!("Event id {} does not match any events stored", event_id);
    }
}

fn change_name(event: &mut Event) {
    print!("Enter new event name: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut new_name = String::new();
    if io::stdin().read_line(&mut new_name).is_ok() {
        event.set_name(new_name.trim().to_string());
        println!("Event name updated to: {}", event.get_name());
    } else {
        println!("Error reading input for name.");
    }

}

fn change_start_time(event: &mut Event) {
    print!("Enter new start time (MM-DD-YYYY HH:MM): ");
    io::stdout().flush().expect("Failed to flush stdout");

    if let Some(new_time) = get_unix_timestamp_from_input() {
        event.set_start_time(new_time);
        println!("Start time updated to: {}", event.get_start_time());
    }
}

fn change_end_time(event: &mut Event) {
    print!("Enter new end time (MM-DD-YYYY HH:MM): ");
    io::stdout().flush().expect("Failed to flush stdout");

    if let Some(new_time) = get_unix_timestamp_from_input() {
        event.set_end_time(new_time);
        println!("End time updated to: {}", event.get_end_time());
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

fn change_note(event: &mut Event) {
    print!("Enter new note: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut new_note = String::new();
    if let Err(e) = io::stdin().read_line(&mut new_note) {
       println!("Error reading input: {}", e);
       return;
    }
    event.set_note(new_note.trim().to_string());
    println!("Note updated.");
}

fn delete_event(planner: &mut Planner, event_id: usize) {
    print!("Are you sure you want to delete event ID {}? (y/n):\n> ", event_id);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut confirmation = String::new();
    if io::stdin().read_line(&mut confirmation).is_err() {
        println!("Error reading input.");
        return;
    }
    match confirmation.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            if let Some(_deleted_event) = planner.delete_event(event_id) {
                println!("Event deleted successfully.");
            } else {
                println!("Failed to delete event. Make sure the event ID is valid.");
            }
        },
        _ => println!("Deletion canceled."),
    }
}