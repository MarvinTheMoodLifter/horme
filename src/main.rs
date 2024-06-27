use std::io::{stdin, stdout, Write};

use color_eyre::Result;

mod app;
mod errors;
mod tui;

use app::App;
use horme::Task;

fn main() -> Result<()> {
    //let mut todo_list: Vec<Task> = Vec::new();
    //run_prompt(&mut todo_list);
    errors::install_hooks()?;
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}

// run the prompt loop in command line interface
#[allow(dead_code)]
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
