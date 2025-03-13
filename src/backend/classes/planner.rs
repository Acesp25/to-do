use crate::event::Event;
use chrono::NaiveDateTime;

pub struct Planner {
    name: String,
    events: Vec<Event>,
    next_event_id: usize,
    event_count: u32
}

impl Planner {
    pub fn new(name: String) -> Self {
        Self {
            name,
            events: Vec::new(),
            next_event_id: 0,
            event_count: 0
        }
    }

    pub fn add_event(&mut self, new_event: Event) {
        self.events.push(new_event);
        self.next_event_id += 1;
        self.event_count += 1;
    }

    pub fn delete_event(&mut self, event_id: usize) -> Option<Event> {
        if event_id < self.events.len() {
            Some(self.events.remove(event_id));
            self.event_count -= 1;
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
        priority: crate::event::Priority,
        reoccurance: crate::event::Reoccurance,
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
        self.add_event(event);
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