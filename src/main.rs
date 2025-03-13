use chrono::NaiveDateTime;
use crate::event::{Event, Priority, Reoccurance};

fn main() {
    let dummy_time = NaiveDateTime::from_timestamp(1_600_000_000, 0);

    let mut e1 = Event::new(
        "Test Event".to_string(),
        dummy_time,
        dummy_time,
        Priority::Medium,
        Reoccurance::None,
        "This is a test note.".to_string(),
        false,
    );

    e1.display();

    println!("Original name: {}", e1.get_name());
    e1.set_name("Updated Event Name".to_string());
    println!("Updated name: {}", e1.get_name());

    e1.display();
}