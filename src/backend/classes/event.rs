use chrono::NaiveDateTime;
use crate::backend::enums::reoccurance::Reoccurance;
use crate::backend::enums::priority::Priority;

#[derive(Debug, Clone)]
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
    // Constructor
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
        println!("| Event ID: {:<24} |", self.id);
        println!("| Event: {:<30} |", self.name);
        println!("| Start Time: {:<24} |", self.start_time);
        println!("| End Time: {:<24} |", self.end_time);
        println!("| Priority: {:<30} |", format!("{:?}", self.priority));
        println!("| Reoccurance: {:<26} |", format!("{:?}", self.reoccurance));
        println!("| Note: {:<30} |", self.note);
        println!("| Completed: {:<27} |", self.completed);
        println!(" __________________________________________");
    }

    pub fn to_string(&self) -> String {
        // Store the timestamps as i64 integers.
        format!(
            "{}|{}|{}|{}|{:?}|{:?}|{}|{}",
            self.id,
            self.name,
            self.start_time.timestamp(),
            self.end_time.timestamp(),
            self.priority,
            self.reoccurance,
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
        let start_ts = parts[2].parse::<i64>().ok()?;
        let end_ts = parts[3].parse::<i64>().ok()?;
        let start_time = NaiveDateTime::from_timestamp_opt(start_ts, 0)?;
        let end_time = NaiveDateTime::from_timestamp_opt(end_ts, 0)?;
        let priority = match parts[4].to_lowercase().as_str() {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::Medium, // default fallback
        };
        let reoccurance = match parts[5].to_lowercase().as_str() {
            "none" => Reoccurance::None,
            "daily" => Reoccurance::Daily,
            "weekly" => Reoccurance::Weekly,
            "monthly" => Reoccurance::Monthly,
            "yearly" => Reoccurance::Yearly,
            "fornite" => Reoccurance::Fornite,
            _ => Reoccurance::None, // default fallback
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