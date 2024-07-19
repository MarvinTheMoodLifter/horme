use color_eyre::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

mod app;
mod task;
mod tui;

use task::Status;
use task::Task;

use app::App;
use tui::{init_error_hooks, init_terminal, restore_terminal};

fn main() -> Result<()> {
    init_error_hooks()?;
    let terminal = init_terminal()?;

    // Build the list of tasks from a markdown file
    // File todo.md must exist in the current path
    let file_path = Path::new("todo.md");

    let contents = fs::read_to_string(file_path).expect(
        "Something went wrong reading the file, check if todo.md exists in the current path.",
    );

    // Define a regular expression pattern for headings (#)
    let heading_pattern = Regex::new(r"^(#{1,6}) (.*)$").unwrap();

    // Initialize the variables to store the current heading and lines after each heading
    let mut current_heading: Option<String> = None;
    let mut todo_list: Vec<Task> = vec![];
    let mut current_status: String = "Todo".to_string();

    // Iterate over each line in the file
    for line in contents.lines() {
        if let Some(captures) = heading_pattern.captures(line) {
            // If a heading is found, store its text and reset the lines vector
            // Print the vurrent heading and lines after each heading
            for line in todo_list.iter() {
                println!("task: {:?}", line);
            }
            current_heading = Some(captures[2].to_string());
            println!(
                "Current heading: {:?}",
                current_heading.as_ref().unwrap().trim_start_matches('#')
            );
            current_status = current_heading.as_ref().unwrap().to_string();
        } else if line.starts_with("- ") {
            // If the line starts with "- ", append it to the lines vector after a heading is found
            if let Some(heading) = &current_heading {
                todo_list.push(Task::new(
                    line[2..].to_string(),
                    "".to_string(),
                    Status::Todo,
                    None,
                ));
            }
            match current_status.as_str() {
                "Todo" => {
                    todo_list.last_mut().unwrap().update_status(Status::Todo);
                }
                "Doing" => {
                    todo_list.last_mut().unwrap().update_status(Status::Doing);
                }
                "Done" => {
                    todo_list.last_mut().unwrap().update_status(Status::Done);
                }
                _ => {}
            }
        } else if line.starts_with("    >") {
            // If the line starts with "    >", append it to the last element in the lines vector
            if let Some(heading) = &current_heading {
                if let Some(last_element) = todo_list.last_mut() {
                    let description = line[5..].trim();
                    last_element.add_description(description);
                }
            }
        }
    }

    // Print all element in the list
    for line in todo_list.iter() {
        println!("task: {:?}", line);
    }

    let mut app = App::new(todo_list);

    app.run(terminal)?;
    restore_terminal()?;

    Ok(())
}
