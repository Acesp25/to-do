use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use chrono::NaiveDateTime;
use crate::backend::classes::event::Event;
use crate::backend::enums::{priority::Priority, reoccurance::Reoccurance};

const FILENAME: &str = "events.txt";

pub fn load_events() -> io::Result<Vec<Event>> {
    let file = File::open(FILENAME)?;
    let reader = BufReader::new(file);
    let mut events = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if let Some(event) = Event::from_string(&line) {
            events.push(event);
        }
    }
    Ok(events)
}

pub fn append_event(event: &Event) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(FILENAME)?;
    writeln!(file, "{}", event.to_string())?;
    Ok(())
}

pub fn delete_event(event_id: usize) -> io::Result<()> {
    let events = load_events()?;
    let filtered: Vec<String> = events
        .into_iter()
        .filter(|e| e.id != event_id)
        .map(|e| e.to_string())
        .collect();
    let mut file = File::create(FILENAME)?;
    for line in filtered {
        writeln!(file, "{}", line)?;
    }
    Ok(())
}