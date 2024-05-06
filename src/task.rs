// Atomic U16 is used to generate unique IDs
use std::sync::atomic::{AtomicU16, Ordering};
static UNIQUE_ID: AtomicU16 = AtomicU16::new(0);

// Task struct
#[derive(Debug, PartialEq)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub id: u16,
    pub due_date: String,
}

impl Task {
    // create a new task with a name and description
    // if id provided, use that id, otherwise generate a new id
    pub fn new(name: String, description: String, id: Option<u16>) -> Self {
        // If id is provided, use that id, otherwise generate a new id
        let new_id = match id {
            Some(id) => id,
            None => UNIQUE_ID.fetch_add(1, Ordering::SeqCst),
        };
        Self {
            name,
            description,
            completed: false,
            id: new_id,
            due_date: "".to_string(),
        }
    }
    // update the task status
    pub fn update_status(&mut self) {
        self.completed = !self.completed;
    }
}
