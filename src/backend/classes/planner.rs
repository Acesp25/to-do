use chrono::NaiveDateTime;

use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

use crate::backend::classes::event::Event;
use crate::backend::enums::priority::Priority;
use crate::backend::enums::reoccurance::Reoccurance;

pub struct Planner {
    name: String,
    events: Vec<Event>,
    next_event_id: usize,
    event_count: u32,
    file_name: String
}

impl Planner {
    pub fn new(name: String) -> Self {
        let file_name = format!("src/planners/{}.txt", name);
        Self {
            name,
            events: Vec::new(),
            next_event_id: 0,
            event_count: 0,
            file_name,
        }
    }

    pub fn add_event(&mut self, new_event: Event) {
        self.events.push(new_event);
        self.next_event_id += 1;
        self.event_count += 1;
    }

    pub fn delete_event(&mut self, event_id: usize) -> Option<Event> {
        if event_id < self.events.len() {
            self.event_count -= 1;
            if let Err(e) = self.delete_event_in_file(event_id) {
                eprintln!("Failed to delete event in file: {}", e);
            }
            Some(self.events.remove(event_id))
        } else {
            println!("Invalid event id {}.", event_id);
            None
        }
    }

    pub fn create_event(
        &mut self,
        name: String,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        priority: Priority,
        reoccurance: Reoccurance,
        note: String,
        completed: bool,
    ) {
        let event = Event::new(
            self.next_event_id,
            name,
            start_time,
            end_time,
            priority,
            reoccurance,
            note,
            completed,
        );
        if let Err(e) = self.append_event_to_file(&event) {
            eprintln!("Failed to append event to file: {}", e);
        }
        self.add_event(event);
    }

    pub fn list_events(&self) {
        if self.events.is_empty() {
            println!("No events schedued!");
        } else {
            for (_id,event) in self.events.iter().enumerate() {
                event.display();
            }
        }
    }

    pub fn display_todays_events(&self, today: NaiveDateTime) {
        let mut found = false;
        for event in &self.events {
            if event.get_start_time().date() == today.date() {
                event.display();
                found = true;
            }
        }
        if !found {
            println!("No events found for today!");
        }
    }

    pub fn find_event_mut(&mut self, id: usize) -> Option<&mut Event> {
        self.events.get_mut(id)
    }

    pub fn load_events_file(&self) -> io::Result<Vec<Event>> {
        let file = File::open(&self.file_name)?;
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
    
    pub fn append_event_to_file(&self, event: &Event) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_name)?;
        writeln!(file, "{}", event.to_string())?;
        Ok(())
    }
    
    pub fn delete_event_in_file(&self, event_id: usize) -> io::Result<()> {
        let events = self.load_events_file()?;
        let filtered: Vec<String> = events
            .into_iter()
            .filter(|e| *e.get_id() != event_id)
            .map(|e| e.to_string())
            .collect();
        let mut file = File::create(&self.file_name)?;
        for line in filtered {
            writeln!(file, "{}", line)?;
        }
        Ok(())
    }
}