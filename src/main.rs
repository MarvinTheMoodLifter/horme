extern crate horme;
use std::io::{stdin, stdout, Write};

use horme::Task;

fn run_prompt(todo_list: &mut Vec<Task>) {
    loop {
        let mut stdout = stdout();
        print!("> ");
        stdout.flush().expect("Could not flush stdout");

        let mut buffer = String::new();
        stdin().read_line(&mut buffer).expect("Cannot read line!");

        let args: Vec<&str> = buffer.split_whitespace().collect();

        horme::run(args, todo_list);
    }
}

fn main() {
    let mut todo_list: Vec<Task> = Vec::new();
    run_prompt(&mut todo_list);
}
