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
pub use task::Task;
pub use tui::Tui;

// ---------------------------------- TESTS ----------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task() {
        let task = Task::new("task".to_string(), "description".to_string(), None);
        assert_eq!(task.name, "task");
    }

    #[test]
    #[ignore]
    fn run_add_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        run(Vec::from(["add", "task", "description"]), &mut todo_list);
        assert_eq!(
            todo_list[0],
            Task::new("task".to_string(), "description".to_string(), Some(0))
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
        let task = Task::new("task".to_string(), "description".to_string(), None);
        todo_list.push(task);
        run(Vec::from(["list"]), &mut todo_list);
    }

    #[test]
    fn run_list_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new("task".to_string(), "description".to_string(), None);
        todo_list.push(task);
        run(Vec::from(["list", "task"]), &mut todo_list);
        assert_eq!(todo_list[0].name, "task");
    }

    #[test]
    fn update_status() {
        let mut task = Task::new("task".to_string(), "description".to_string(), None);
        task.update_status();
        assert_eq!(task.completed, true);
    }

    #[test]
    fn run_edit() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new("task".to_string(), "description".to_string(), None);
        todo_list.push(task);
        run(
            Vec::from(["edit", "0", "task", "description"]),
            &mut todo_list,
        );
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into()).unwrap();
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert_eq!(app.exit, true);
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.handle_key_event(KeyCode::Left.into());
    }

    #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert_eq!(
            app.handle_key_event(KeyCode::Right.into())
                .unwrap_err()
                .to_string(),
            "counter overflow"
        );
    }

    #[test]
    #[ignore]
    fn render() {
        let app = App::default();
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
