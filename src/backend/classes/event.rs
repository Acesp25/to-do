use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::backend::enums::reoccurance::Reoccurance;
use crate::backend::enums::priority::Priority;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    id: usize,
    name: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    priority: Priority,
    reoccurance: Reoccurance,
    note: String,
    completed: bool,
}

impl Event {
    pub fn new(
        id: usize,
        name: String,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        priority: Priority,
        reoccurance: Reoccurance,
        note: String,
        completed: bool,
    ) -> Self {
        // Validate that end_time is after start_time
        if end_time < start_time {
            eprintln!("Warning: End time is before start time");
        }

        Self {
            id,
            name,
            start_time,
            end_time,
            priority,
            reoccurance,
            note,
            completed,
        }
    }
    
    // Helpers
    pub fn display(&self) {
        println!(" __________________________________________");
        println!("| Event ID: {}", self.id);
        println!("| Event: {}", self.name);
        println!("| Start Time: {}", self.start_time);
        println!("| End Time: {}", self.end_time);
        println!("| Priority: {}", format!("{:?}", self.priority));
        println!("| Reoccurance: {}", format!("{:?}", self.reoccurance));
        println!("| Note: {}", self.note);
        println!("| Completed: {}", self.completed);
        println!(" __________________________________________");
    }

    pub fn to_string(&self) -> String {
        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}",
            self.id,
            self.name,
            self.start_time.format("%Y-%m-%d %H:%M:%S"),
            self.end_time.format("%Y-%m-%d %H:%M:%S"),
            format!("{:?}", self.priority),
            format!("{:?}", self.reoccurance),
            self.note,
            self.completed
        )
    }

    pub fn from_string(s: &str) -> Option<Event> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 8 {
            return None;
        }

        let id = parts[0].parse::<usize>().ok()?;
        let name = parts[1].to_string();
        
        let start_time = NaiveDateTime::parse_from_str(parts[2], "%Y-%m-%d %H:%M:%S")
            .map_err(|e| {
                eprintln!("Error parsing start time: {}", e);
                e
            }).ok()?;
            
        let end_time = NaiveDateTime::parse_from_str(parts[3], "%Y-%m-%d %H:%M:%S")
            .map_err(|e| {
                eprintln!("Error parsing end time: {}", e);
                e
            }).ok()?;

        let priority = match parts[4] {
            "High" => Priority::High,
            "Medium" => Priority::Medium,
            "Low" => Priority::Low,
            _ => Priority::Medium,
        };

        let reoccurance = match parts[5] {
            "None" => Reoccurance::None,
            "Daily" => Reoccurance::Daily,
            "Weekly" => Reoccurance::Weekly,
            "Monthly" => Reoccurance::Monthly,
            "Yearly" => Reoccurance::Yearly,
            "Fornite" => Reoccurance::Fornite,
            _ => Reoccurance::None,
        };

        let note = parts[6].to_string();
        let completed = parts[7].parse::<bool>().unwrap_or(false);

        Some(Event::new(id, name, start_time, end_time, priority, reoccurance, note, completed))
    }

    // Getters
    pub fn get_id(&self) -> &usize {
        &self.id
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn get_start_time(&self) -> &NaiveDateTime {
        &self.start_time
    }
    pub fn get_end_time(&self) -> &NaiveDateTime {
        &self.end_time
    }
    pub fn get_reoccurance(&self) -> &Reoccurance {
        &self.reoccurance
    }
    pub fn get_note(&self) -> &String {
        &self.note
    }
    pub fn get_completed(&self) -> bool {
        self.completed
    }

    // Setters
    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn set_start_time(&mut self, new_start_time: NaiveDateTime) {
        self.start_time = new_start_time;
    }
    pub fn set_end_time(&mut self, new_end_time: NaiveDateTime) {
        self.end_time = new_end_time;
    }
    pub fn set_reoccurance(&mut self, new_reoccurance: Reoccurance) {
        self.reoccurance = new_reoccurance;
    }
    pub fn set_note(&mut self, new_note: String) {
        self.note = new_note;
    }
    pub fn set_completed(&mut self, new_completed: bool) {
        self.completed = new_completed;
    }
}