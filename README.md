<div align="center">

[<img src="https://github.com/MarvinTheMoodLifter/horme/blob/main/assets/horme-banner.png?raw=true" width="500" />](https://github.com/MarvinTheMoodLifter/horme/wiki)

A simple command-line ToDo list application.<br>
*Check the [Wiki](https://github.com/MarvinTheMoodLifter/horme/wiki) for more details.*

</div>

> [!WARNING]
> This project is currently in a **alpha state**, meaning it's still under heavy development and many features are missing or in development.<br>
> Consider this a WIP!

> [!NOTE]
> Also keep in mind that this is my first real project, so the polish of the code might be lacking.
> Feel free to contribute your ideas by sending pull requests or issues on Github.

---
## 💠 Features

- TUI ([Ratatui](https://ratatui.rs))
- Vim keys
- Display the to-do list
- Edit tasks name and description
- Mark tasks as to-do, in progress or done
- Save and load your to-do list in markdown format for easy access and sharing

## 🧪 Upcoming
* [ ] Add a new task. You can specify the name and description of the task
* [ ] Help overlay
* [ ] Delete a task
* [ ] Tasks deadlines
* [ ] Filter tasks by completion status, due date, and overdue status
* [ ] Divide the tasks into user defined categories
* [ ] Customizable theme
* [ ] Add subtasks to a task
* [ ] Improve error handling and user input validation for robustness and user-friendliness
* [ ] Encryption support

## 📦 Install

### Prerequisites
To build and run Horme, make sure you have Rust installed on your system. [Here](https://www.rust-lang.org/tools/install) you can find the official guide to install the toolchain.

### Building from source
```bash
git clone https://github.com/MarvinTheMoodLifter/horme.git
cd horme

cargo build --release
```

Once built, you can find the executable binary in the "target/release" directory.
Just execute the binary:

```bash
./target/release/horme
```

## 🐞 Bug reports

Found a bug or have a feature request? please open an [issue](https://github.com/MarvinTheMoodLifter/horme/issues/new).

## 📜 License

This project is licensed under the MIT license. For more details, see the [LICENSE](https://github.com/MarvinTheMoodLifter/horme/blob/main/LICENSE) file.
