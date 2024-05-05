pub fn run(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    match args[0] {
        "add" => {
            if let Some(name) = args.get(1) {
                if let Some(description) = args.get(2) {
                    let task = Task::new(name.to_string(), description.to_string());
                    todo_list.push(task);
                }
            } else {
                println!("Please provide a task name and description");
            }
        }
        "list" => {
            if todo_list.is_empty() {
                println!("No tasks found");
            }
            for task in todo_list {
                println!("{}  {}", task.id, task.name);
            }
        }
        "edit" => {
            panic!("Please provide a task id");
        }
        "delete" => {
            if let Some(id) = args.get(1) {
                if let Ok(id) = id.parse::<u64>() {
                    todo_list.retain(|task| task.id != id);
                }
            } else {
                panic!("Please provide a task id");
            }
        }
        "complete" => {}
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

#[derive(Debug, PartialEq)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub id: u64,
    pub due_date: String,
}

impl Task {
    // create a new task
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            completed: false,
            id: 0,
            due_date: "".to_string(),
        }
    }

    // update the task status
    pub fn update_status(&mut self) {
        self.completed = !self.completed;
    }

    // update the task name
    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }
}

pub fn help_message() {
    // Show the list of commands and their usage
    println!("Commands:");
    println!("  add <name> <description>         - Add a new task");
    println!("  list                             - List all tasks");
    println!("  list --completed                 - List all completed tasks");
    println!("  list --incomplete                - List all incomplete tasks");
    println!("  list --due <date>                - List all tasks due on <date>");
    println!("  list --overdue                   - List all tasks overdue");
    println!("  edit <id> <name> <description>   - Edit a task");
    println!("  delete <id>                      - Delete a task");
    println!("  complete <id>                    - Complete a task");
    println!("  exit                             - Exit the program");
    println!("  help                             - Show this help message");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task() {
        let task = Task::new("task".to_string(), "description".to_string());
        assert_eq!(task.name, "task");
    }

    #[test]
    fn run_add_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new("task".to_string(), "description".to_string());
        run(Vec::from(["add", "task", "description"]), &mut todo_list);
        assert_eq!(todo_list[0], task);
    }

    #[test]
    fn run_add_no_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        run(Vec::from(["add"]), &mut todo_list);
    }

    #[test]
    fn run_list_task() {
        let mut todo_list: Vec<Task> = Vec::new();
        let task = Task::new("task".to_string(), "description".to_string());
        todo_list.push(task);
    }

    #[test]
    fn update_status() {
        let mut task = Task::new("task".to_string(), "description".to_string());
        task.update_status();
        assert_eq!(task.completed, true);
    }

    #[test]
    fn update_name() {
        let mut task = Task::new("task".to_string(), "description".to_string());
        task.update_name("new task".to_string());
        assert_eq!(task.name, "new task");
    }
}
