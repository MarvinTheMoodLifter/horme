<div align="center">

[<img src="assets/horme-banner.png">](https://github.com/MarvinTheMoodLifter/horme/wiki)

A simple command-line ToDo list application.<br>
*Check the [Wiki](https://github.com/MarvinTheMoodLifter/horme/wiki) for more details.*

</div>

> [!WARNING]
> This project is currently in a **pre-alpha state**, meaning it's still under heavy development and many features are missing or in development.<br>
> Consider this a WIP!

---
## 📦 Install
To build and run Horme, make sure you have Rust installed on your system. [Here](https://www.rust-lang.org/tools/install) you can find the official guide to install the toolchain.

Then, clone this repository and navigate to the project directory:
```bash
git clone https://github.com/MarvinTheMoodLifter/horme.git
cd horme

cargo build --release
```

Once built, you can find the executable binary in the "target/release" directory.
You run the program by executing the binary:
```bash
./target/release/horme
```

## 💠 Features
- `Add`: Add a new task to the todo list. You can specify the name and description of the task.
- `List`: Display the list of tasks. You can optionally filter tasks by completion status, due date, or overdue status.
- `Edit`: Edit a task by providing its ID along with the updated name and description.
- `Delete`: Delete a task by providing its ID.
- `Complete`: Mark a task as completed by providing its ID.
- `Exit`: Exit the program.
- `Help`: Display a help message with a list of available commands and their usage.

## 🧪 Upcoming
* [ ] Add support for displaying tasks in a Terminal User Interface using [Ratatui](https://ratatui.rs).
* [ ] Implement functionality to filter tasks by completion status, due date, and overdue status in the "list" command.
* [ ] Divide the tasks into user defined categories.
* [ ] Implement functionality to edit a task's due date.
* [ ] Add support for persisting tasks between sessions, e.g., using a file or a database.
* [ ] Improve error handling and user input validation for robustness and user-friendliness.

