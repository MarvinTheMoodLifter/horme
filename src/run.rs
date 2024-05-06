use crate::task::Task;
use crate::user_interactions;

pub fn run(args: Vec<&str>, todo_list: &mut Vec<Task>) {
    match args[0] {
        "add" => {
            user_interactions::add_task(args, todo_list);
        }
        "list" | "ls" => {
            user_interactions::list_tasks(args, todo_list);
        }
        "edit" => {
            user_interactions::edit_task(args, todo_list);
        }
        "delete" => {
            user_interactions::delete_task(args, todo_list);
        }
        "complete" => {
            user_interactions::complete_task(args, todo_list);
        }
        "exit" => {
            std::process::exit(0);
        }
        "help" => {
            user_interactions::help_message();
        }
        _ => {
            println!("Invalid command. Type <help> for a list of commands");
        }
    }
}
