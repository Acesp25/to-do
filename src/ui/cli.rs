use crate::ui::ui_helpers::{display_menu, get_choice, event_creater, adjust_event};
use crate::backend::classes::planner::{self, Planner};
use chrono::Local;

pub fn start_ui() {
    let mut planner = Planner::new("Test Planner".to_string());

    loop {
        let current_date_time = Local::now().naive_local();
        display_menu(current_date_time);
        match get_choice() {
            Some(1) => planner.display_todays_events(current_date_time),
            Some(2) => event_creater(&mut planner),
            Some(3) => adjust_event(&mut planner),
            Some(4) => planner.list_events(),
            Some(5) => break,
            _ => println!("Invalid choice, please try again."),
        }
    }
}