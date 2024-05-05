# Horme

Horme is a simple command-line ToDo list application written in Rust.

## Features
- Add: Add a new task to the todo list. You can specify the name and description of the task.
- List: Display the list of tasks. You can optionally filter tasks by completion status, due date, or overdue status.
- Edit: Edit a task by providing its ID along with the updated name and description.
- Delete: Delete a task by providing its ID.
- Complete: Mark a task as completed by providing its ID.
- Exit: Exit the program.
- Help: Display a help message with a list of available commands and their usage.

### ToDo
- Implement functionality to filter tasks by completion status, due date, and overdue status in the "list" command.
- Implement functionality to edit a task's due date.
- Add support for persisting tasks between sessions, e.g., using a file or a database.
- Improve error handling and user input validation for robustness and user-friendliness.
- Add support for displaying tasks in a graphical user interface using [Iced](https://iced.rs/).

## Installation
To build and run Horme, make sure you have Rust installed on your system. Then, clone this repository and navigate to the project directory:
```
git clone https://github.com/MarvinTheMoodLifter/horme.git
cd horme

cargo build --release
```

Once built, you can find the executable binary in the "target/release" directory.
You run the program by executing the binary:
```
./target/release/horme
```
