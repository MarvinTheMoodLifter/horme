// Atomic U16 is used to generate unique IDs
use std::fmt::Display;
use std::sync::atomic::{AtomicU16, Ordering};
static UNIQUE_ID: AtomicU16 = AtomicU16::new(0);

// Task struct
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub id: u16,
    pub due_date: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum Status {
    Todo,
    Doing,
    Done,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "Todo"),
            Status::Doing => write!(f, "Doing"),
            Status::Done => write!(f, "Done"),
        }
    }
}

impl Task {
    // create a new task with a name and description
    // if id provided, use that id, otherwise generate a new id
    pub fn new(name: String, description: String, status: Status, id: Option<u16>) -> Self {
        // If id is provided, use that id, otherwise generate a new id
        let new_id = match id {
            Some(id) => id,
            None => UNIQUE_ID.fetch_add(1, Ordering::SeqCst),
        };
        Self {
            name,
            description,
            status,
            id: new_id,
            due_date: "".to_string(),
        }
    }

    // update the task status
    pub fn update_status(&mut self) {
        match self.status {
            Status::Todo => self.status = Status::Doing,
            Status::Doing => self.status = Status::Done,
            Status::Done => self.status = Status::Todo,
        }
    }

    pub fn add_description(&mut self, description: &str) {
        self.description.push_str(description);
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_status(&self) -> Status {
        self.status
    }
}
