use color_eyre::Result;

mod app;
mod task;
mod tui;

use app::App;

fn main() -> Result<()> {
    // run_prompt(&mut todo_list);
    // errors::install_hooks()?;
    tui::init_error_hooks()?;
    let terminal = tui::init_terminal()?;

    let mut app = App::default();

    app.run(terminal)?;
    tui::restore_terminal()?;

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
