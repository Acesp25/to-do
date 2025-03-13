use crate::event::Event;
use chrono::NaiveDateTime;

pub struct Planner {
    name: String,
    events: Vec<Event>,
}

impl Planner {
    pub fn new(name: String) -> Self {
        Self {
            name,
            events: Vec::new(),
        }
    }

    pub fn add_event(&mut self, new_event: Event) {
        self.events.push(event);
    }

    pub fn list_events(&self) {
        if self.events.is_empty() {
            println!("No events schedued!");
        } else {
            for (id,event) in self.events.iter().enumerate() {
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
}