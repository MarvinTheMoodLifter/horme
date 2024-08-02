mod app;
mod colors;
mod task;
mod tui;

pub use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
pub use ratatui::prelude::*;

pub use app::App;
pub use task::Status;
pub use task::Task;

// ---------------------------------- TESTS ----------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task() {
        let task = Task::new(
            "task".to_string(),
            "description".to_string(),
            Status::Todo,
            None,
        );
        assert_eq!(task.name, "task");
    }
}
