mod app;
mod errors;
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

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert_eq!(app.should_exit, true);
    }

    #[test]
    #[ignore]
    fn render() {
        let mut app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━━━━ Horme ━━━━━━━━━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(21, 0, 7, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        // note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
        assert_eq!(buf, expected);
    }
}
