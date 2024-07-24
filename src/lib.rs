mod app;
mod run;
mod task;
mod tui;
mod user_interactions;
mod utils;

pub use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
pub use ratatui::prelude::*;

pub use app::App;
pub use run::run;
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

    #[test]
    #[ignore]
    fn run_add_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        run(Vec::from(["add", "task", "description"]), &mut todo_list);
        assert_eq!(
            todo_list[0],
            Task::new(
                "task".to_string(),
                "description".to_string(),
                Status::Todo,
                Some(0)
            )
        );
    }

    #[test]
    fn run_add_no_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        run(Vec::from(["add"]), &mut todo_list);
    }

    #[test]
    fn run_list_no_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        run(Vec::from(["list"]), &mut todo_list);
    }

    #[test]
    fn run_list_all() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new(
            "task".to_string(),
            "description".to_string(),
            Status::Todo,
            None,
        );
        todo_list.push(task);
        run(Vec::from(["list"]), &mut todo_list);
    }

    #[test]
    fn run_list_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new(
            "task".to_string(),
            "description".to_string(),
            Status::Todo,
            None,
        );
        todo_list.push(task);
        run(Vec::from(["list", "task"]), &mut todo_list);
        assert_eq!(todo_list[0].name, "task");
    }

    #[test]
    fn run_edit() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new(
            "task".to_string(),
            "description".to_string(),
            Status::Todo,
            None,
        );
        todo_list.push(task);
        run(
            Vec::from(["edit", "0", "task", "description"]),
            &mut todo_list,
        );
    }
}
