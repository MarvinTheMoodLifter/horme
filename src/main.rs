use gtk::{prelude::*, ListBox};
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use std::io::{stdin, stdout, Write};

mod screen;
mod theme;

use horme::Task;

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

fn main() {
    //let mut todo_list: Vec<Task> = Vec::new();
    //run_prompt(&mut todo_list);
    let app = Application::new(Some("dev.marcoperin.horme"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Horme"));
        window.set_default_size(800, 600);

        let main_box = Box::new(Orientation::Horizontal, 5);

        // Left column
        let left_box = Box::new(Orientation::Vertical, 5);
        let categories_label = Label::new(Some("Categories"));
        let categories_list = ListBox::new();
        let add_category_button = Button::with_label("Add Category");
        left_box.append(&categories_label);
        left_box.append(&categories_list);
        left_box.append(&add_category_button);

        // Center column
        let center_box = Box::new(Orientation::Vertical, 5);
        let todos_label = Label::new(Some("Todo"));
        let todos_list = ListBox::new();
        let add_todo_button = Button::with_label("Add Task");
        center_box.append(&todos_label);
        center_box.append(&todos_list);
        center_box.append(&add_todo_button);

        // Right column
        let right_box = Box::new(Orientation::Vertical, 5);
        let details_label = Label::new(Some("Details"));
        right_box.append(&details_label);

        main_box.append(&left_box);
        main_box.append(&center_box);
        main_box.append(&right_box);

        window.set_child(Some(&main_box));

        ApplicationWindow::set_visible(&window, true);
    });

    app.run();
}
