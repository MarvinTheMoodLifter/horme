use crate::task::Task;
use crate::utils;

// Add a new task
pub fn add_task(args: Vec<&str>, todo_list: &mut Vec<Task>) {
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

// Print tasks in a nice table
pub fn list_tasks(args: Vec<&str>, todo_list: &Vec<Task>) {
    if todo_list.is_empty() {
        println!("No tasks found");
        return;
    }

    for task in todo_list {
        if args.len() == 1 || args.contains(&"--all") || args.contains(&"-a") {
            utils::print_task(task);
        } else if args.contains(&"--completed") || args.contains(&"-c") && task.completed {
            utils::print_task(task);
        } else if args.contains(&"--incomplete") || args.contains(&"-i") && !task.completed {
            utils::print_task(task);
        } else if args.contains(&"--due") || args.contains(&"-d") && task.due_date == args[2] {
            utils::print_task(task);
        }
    }
}

// Delete a tasks
pub fn delete_task(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    // Check if id is present in todo_list
    if let Some(id) = args.get(1) {
        if let Ok(id) = id.parse::<u16>() {
            if utils::check_id(id, todo_list) {
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

// Edit a task
pub fn edit_task(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    let mut task_found = false;
    for task in todo_list {
        if task.id == args[1].parse::<u16>().unwrap() {
            task_found = true;
            match args[2] {
                "--name" | "-n" => {
                    task.name = args[3].to_string();
                }
                "--description" | "-d" => {
                    task.description = args[3].to_string();
                }
                _ => {
                    println!("Please provide a valid task id (run <list> to see task ids)");
                }
            }
            break;
        }
    }
    if !task_found {
        println!("Please provide a valid task id (run <list> to see task ids)");
    }
}

// Complete a task
pub fn complete_task(args: Vec<&str>, todo_list: &mut Vec<Task>) {
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

pub fn help_message() {
    // Show the list of commands and their usage
    println!("Commands:");
    println!("  add <name> <description>                            - Add a new task");
    println!("  list, ls <OPTION>                                   - List all tasks");
    println!("      -c  --completed                                 - List all completed tasks");
    println!("      -i  --incomplete                                - List all incomplete tasks");
    println!(
        "      -d  --due <date>                                - List all tasks due on <date>"
    );
    println!("  edit <id> <OPTION> <new_value>                      - Edit a task");
    println!("      -n  --name <new_name>                           - Edit task name");
    println!("      -d  --description <new_description>             - Edit task description");
    println!("  delete <id>                                         - Delete a task");
    println!("  complete <id>                                       - Complete a task");
    println!("  exit                                                - Exit the program");
    println!("  help                                                - Show this help message");
}
