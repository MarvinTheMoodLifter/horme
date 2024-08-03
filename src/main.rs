use color_eyre::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

use hocon::HoconLoader;

mod app;
mod colors;
mod task;
mod tui;

use task::Status;
use task::Task;

use app::App;
use tui::{init_error_hooks, init_terminal, restore_terminal};

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // File todo.md must exist in the current path
    let file_path = Path::new("todo.md");

    // Read the config file
    let config = HoconLoader::new()
        .load_file("tests/data/theme.conf")?
        .hocon()?;

    let theme = config["theme"].as_string().unwrap_or("default".to_string());

    // Build the list of tasks from a markdown file
    let todo_list = build_todo_list(file_path)?;

    // Initialize the application
    let mut app = App::new(todo_list, file_path, theme.as_str());

    // Run the application
    app.run(terminal)?;

    restore_terminal()?;

    Ok(())
}

fn build_todo_list(file_path: &Path) -> Result<Vec<Task>> {
    // Build the list of tasks from a markdown file
    let contents = fs::read_to_string(file_path).expect(
        "Something went wrong reading the file, check if todo.md exists in the current path.",
    );

    // Define a regular expression pattern for headings (#)
    let heading_pattern = Regex::new(r"^(#{1,6}) (.*)$").unwrap();

    // Initialize the variables to store the current heading and lines after each heading
    let mut current_heading: Option<String> = None;
    let mut todo_list: Vec<Task> = vec![];

    // Iterate over each line in the file
    for line in contents.lines() {
        if let Some(captures) = heading_pattern.captures(line) {
            // If a heading is found, store its text and reset the lines vector
            current_heading = Some(captures[2].to_string());
        } else if line.starts_with("- ") {
            let status = match &current_heading {
                Some(head) if head.trim().eq_ignore_ascii_case("Todo") => Status::Todo,
                Some(head) if head.trim().eq_ignore_ascii_case("Doing") => Status::Doing,
                Some(head) if head.trim().eq_ignore_ascii_case("Done") => Status::Done,
                _ => Status::Todo, // Default to Todo if no heading is found or doesn't match
            };
            todo_list.push(Task::new(
                line[2..].to_string(),
                String::new(),
                status,
                None,
            ));
        } else if line.starts_with("    >") {
            if let Some(last_element) = todo_list.last_mut() {
                last_element.add_description(&line[5..].trim());
            }
        } else if line.starts_with("    * [ ]") || line.starts_with("    * [x]") {
            if let Some(last_element) = todo_list.last_mut() {
                if line.starts_with("    * [ ]") {
                    last_element.add_subtask(line[9..].trim().to_string(), false);
                } else if line.starts_with("    * [x]") {
                    last_element.add_subtask(line[9..].trim().to_string(), true);
                }
            }
        }
    }

    Ok(todo_list)
}
