use crate::task::Task;

// Check if id is present in todo_list
pub fn check_id(id: u16, todo_list: &Vec<Task>) -> bool {
    todo_list.iter().any(|task| task.id == id)
}

// Print task
pub fn print_task(task: &Task) {
    println!(
        "ID: {} - Name: {}, Description: {}, [Status: {}] Due Date: {}",
        task.id, task.name, task.description, task.completed, task.due_date
    );
}

// get task
#[allow(dead_code)]
pub fn get_task(id: u16, todo_list: &Vec<Task>) -> Option<&Task> {
    todo_list.iter().find(|task| task.id == id)
}
