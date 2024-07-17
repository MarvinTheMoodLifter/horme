use color_eyre::Result;

mod app;
mod task;
mod tui;

use app::App;
use tui::{init_error_hooks, init_terminal, restore_terminal};

fn main() -> Result<()> {
    // run_prompt(&mut todo_list);
    // errors::install_hooks()?;
    init_error_hooks()?;
    let terminal = init_terminal()?;

    let mut app = App::default();

    app.run(terminal)?;
    restore_terminal()?;

    Ok(())
}

// use std::io::{stdin, stdout, Write};
// run the prompt loop in command line interface
// #[allow(dead_code)]
// fn run_prompt(todo_list: &mut Vec<Task>) {
//     loop {
//         let mut stdout = stdout();
//         print!("> ");
//         stdout.flush().expect("Could not flush stdout");
//
//         let mut buffer = String::new();
//         stdin().read_line(&mut buffer).expect("Cannot read line!");
//
//         let args: Vec<&str> = buffer.split_whitespace().collect();
//
//         horme::run(args, todo_list);
//     }
// }
