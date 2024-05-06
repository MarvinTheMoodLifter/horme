pub fn run(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    match args[0] {
        "add" => {
            if let Some(name) = args.get(1) {
                if let Some(description) = args.get(2) {
                    let task = Task::new(name.to_string(), description.to_string(), None);
                    todo_list.push(task);
                } else {
                    let task = Task::new(name.to_string(), "".to_string(), None);
                    todo_list.push(task);
                }
            } else {
                println!("Please provide a task name and description");
            }
        }
        "list" | "ls" => {
            if todo_list.is_empty() {
                println!("No tasks found");
            } else {
                print_taks(todo_list);
            }
        }
        "edit" => {
            edit_task(args, todo_list);
        }
        "delete" => {
            // Check if id is present in todo_list
            if let Some(id) = args.get(1) {
                if let Ok(id) = id.parse::<u16>() {
                    if check_id(id, todo_list) {
                        todo_list.retain(|task| task.id != id);
                    } else {
                        println!("Please provide a valid task id (run <list> to see task ids)");
                    }
                } else {
                    println!("Please provide a valid task id (run <list> to see task ids)");
                }
                // TODO: Add reordering of ids
            } else {
                println!("Please provide a valid task id (run <list> to see task ids)");
            }
        }
        "complete" => {
            // Check if id is present in todo_list and update the status
            if let Some(id) = args.get(1) {
                if let Ok(id) = id.parse::<u16>() {
                    for task in todo_list.iter_mut() {
                        if task.id == id {
                            task.update_status();
                        }
                    }
                }
            } else {
                println!("Please provide a task id");
            }
        }
        "exit" => {
            std::process::exit(0);
        }
        "help" => {
            help_message();
        }
        _ => {
            println!("Invalid command. Type <help> for a list of commands");
        }
    }
}

// Check if id is present in todo_list
fn check_id(id: u16, todo_list: &Vec<Task>) -> bool {
    for task in todo_list {
        if task.id == id {
            return true;
        }
    }
    false
}

// Print tasks in a nice table
fn print_taks(todo_list: &Vec<Task>) {
    for task in todo_list {
        println!("{} {} - {}", task.id, task.name, task.description);
    }
}

// Edit a task with an interactive prompt
fn edit_task(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    // Check with match what the user wants to edit
    match args[2] {
        "--name" | "-n" => {
            if let Some(id) = args.get(1) {
                if let Ok(id) = id.parse::<u16>() {
                    if check_id(id, todo_list) {
                        if let Some(name) = args.get(3) {
                            for task in todo_list.iter_mut() {
                                if task.id == id {
                                    task.name = name.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        "--description" | "-d" => {
            if let Some(id) = args.get(1) {
                if let Ok(id) = id.parse::<u16>() {
                    if check_id(id, todo_list) {
                        if let Some(description) = args.get(3) {
                            for task in todo_list.iter_mut() {
                                if task.id == id {
                                    task.description = description.to_string();
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            println!("Invalid command. Type <help> for a list of commands");
        }
    }
}

// Task struct
#[derive(Debug, PartialEq)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub id: u16,
    pub due_date: String,
}

// Atomic U16 is used to generate unique IDs
use std::sync::atomic::{AtomicU16, Ordering};
static UNIQUE_ID: AtomicU16 = AtomicU16::new(0);

impl Task {
    // create a new task with a name and description
    // if id provided, use that id, otherwise generate a new id
    fn new(name: String, description: String, id: Option<u16>) -> Self {
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

pub fn help_message() {
    // Show the list of commands and their usage
    println!("Commands:");
    println!("  add <name> <description>                        - Add a new task");
    println!("  list, ls <OPTION>                               - List all tasks");
    println!("      --completed                                 - List all completed tasks");
    println!("      --incomplete                                - List all incomplete tasks");
    println!("      --due <date>                                 - List all tasks due on <date>");
    println!("      --overdue                                   - List all tasks overdue");
    println!("  edit <id> <OPTION> <new_value>      - Edit a task");
    println!("      -n  --name <new_name>                           - Edit task name");
    println!("      -d  --description <new_description>             - Edit task description");
    println!("  delete <id>                                     - Delete a task");
    println!("  complete <id>                                   - Complete a task");
    println!("  exit                                            - Exit the program");
    println!("  help                                            - Show this help message");
}

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
    #[should_panic]
    fn run_edit() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new("task".to_string(), "description".to_string(), None);
        todo_list.push(task);
        run(
            Vec::from(["edit", "0", "task", "description"]),
            &mut todo_list,
        );
    }
}
