use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind::*, Color, Modifier, Style, Stylize},
    symbols,
    terminal::Terminal,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

const TODO_HEADER_STYLE: Style = Style::new()
    .fg(SLATE.c800)
    .bg(AMBER.c400)
    .add_modifier(Modifier::BOLD);
const INFO_HEADER_STYLE: Style = Style::new()
    .fg(SLATE.c800)
    .bg(GREEN.c400)
    .add_modifier(Modifier::BOLD);
const SUBTASK_HEADER_STYLE: Style = Style::new()
    .fg(SLATE.c800)
    .bg(RED.c400)
    .add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c800;

use crate::colors::ColorPalette;
use crate::task::{Status, Subtask, Task};

//#[derive(Debug)]
pub struct App {
    pub name_input: String,
    pub description_input: String,
    pub todo_list: TodoList,
    pub subtask_list: SubtaskList,
    pub file_path: PathBuf,
    pub sections_order: Vec<String>,
    pub palette: ColorPalette,
    pub should_exit: bool,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

#[derive(Debug, Default)]
pub struct TodoList {
    pub state: ListState,
    pub items: Vec<Task>,
}

#[derive(Debug, Default)]
pub struct SubtaskList {
    pub state: ListState,
    pub items: Vec<Subtask>,
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Editing,
    Subtask,
    AddTask,
    Deleting,
    AddSubtask,
}

pub enum CurrentlyEditing {
    Name,
    Description,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name_input: String::new(),
            description_input: String::new(),
            should_exit: false,
            file_path: Path::new("default.md").to_path_buf(),
            sections_order: vec![
                "## Todo".to_string(),
                "## Doing".to_string(),
                "## Done".to_string(),
            ],
            todo_list: TodoList::from_iter([
                (
                    "Check synthetic eyes",
                    "Are you sure this isn't a replicant?",
                    Status::Doing,
                ),
                (
                    "Investigate Tyrell Corporation",
                    "The secrets they're hiding...",
                    Status::Todo,
                ),
                (
                    "Complete the Dragonborn questline",
                    "You have what it takes to save Tamriel!",
                    Status::Done,
                ),
                (
                    "Kill as many dragons as possible",
                    "Those fire-breathers need puttin' down!",
                    Status::Todo,
                ),
                (
                    "Take a break, dude",
                    "Life's too short for bowling alleys and White Russians.",
                    Status::Todo,
                ),
                (
                    "Find the missing rug",
                    "Man, that rug really tied the room together...",
                    Status::Done,
                ),
                (
                    "Visit the planet Frogstar World B",
                    "A great place for a holiday... or so I've heard.",
                    Status::Todo,
                ),
                (
                    "Don't forget your towel",
                    "You never know when you might need it!",
                    Status::Done,
                ),
            ]),
            palette: ColorPalette::default(),
            subtask_list: SubtaskList::default(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
}

impl FromIterator<(&'static str, &'static str, Status)> for TodoList {
    // Create a list of tasks from an iterator
    fn from_iter<I: IntoIterator<Item = (&'static str, &'static str, Status)>>(iter: I) -> Self {
        let item = iter
            .into_iter()
            .map(|(name, description, status)| {
                Task::new(name.to_string(), description.to_string(), status, None)
            })
            .collect();
        let state = ListState::default();
        Self { state, items: item }
    }
}

impl FromIterator<&'static str> for SubtaskList {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        let item = iter
            .into_iter()
            .map(|name| Subtask::new(name.to_string()))
            .collect();
        let state = ListState::default();
        Self { state, items: item }
    }
}

impl App {
    pub fn new(todo_list: Vec<Task>, file_path: &Path) -> Self {
        let state = ListState::default();

        Self {
            name_input: String::new(),
            description_input: String::new(),
            todo_list: TodoList {
                state,
                items: todo_list,
            },
            subtask_list: SubtaskList::default(),
            file_path: file_path.to_path_buf(),
            sections_order: vec![
                "## Todo".to_string(),
                "## Doing".to_string(),
                "## Done".to_string(),
            ],
            palette: ColorPalette::default(),
            should_exit: false,
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }
    // runs the application's main loop until the user quits
    pub fn run(&mut self, mut terminal: Terminal<impl Backend>) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|f| f.render_widget(&mut *self, f.size()))?;
            if let Event::Key(key) = event::read()? {
                let _ = self.handle_key_event(key);
            };
        }
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.current_screen {
            CurrentScreen::Main => match key_event.code {
                KeyCode::Char('j') => self.select_next(),
                KeyCode::Char('k') => self.select_previous(),
                KeyCode::Char('c') => self.start_editing(),
                KeyCode::Char('s') => self.start_subtask(),
                KeyCode::Char('a') => self.start_adding(),
                KeyCode::Char('d') => self.start_deleting(),
                KeyCode::Down => self.select_next(),
                KeyCode::Up => self.select_previous(),
                KeyCode::Enter => self.toggle_status(),
                KeyCode::Char('q') => self.exit(),
                _ => {}
            },
            CurrentScreen::Editing => match key_event.code {
                KeyCode::Enter => self.save_edited_task(),
                KeyCode::Esc => self.cancel_editing(),
                KeyCode::Tab => self.toggle_editing_field(),
                KeyCode::Char(c) => self.handle_editing_input(c),
                KeyCode::Backspace => self.handle_backspace(),
                _ => {}
            },
            CurrentScreen::AddTask => match key_event.code {
                KeyCode::Enter => self.add_new_task(),
                KeyCode::Esc => self.cancel_adding(),
                KeyCode::Tab => self.toggle_editing_field(),
                KeyCode::Char(c) => self.handle_editing_input(c),
                KeyCode::Backspace => self.handle_backspace(),
                _ => {}
            },
            CurrentScreen::Deleting => match key_event.code {
                KeyCode::Char('d') => self.delete_task(),
                KeyCode::Esc => self.cancel_deleting(),
                _ => {}
            },
            CurrentScreen::Subtask => match key_event.code {
                KeyCode::Char('n') => self.select_next_subtask(),
                KeyCode::Char('p') => self.select_previous_subtask(),
                KeyCode::Delete => self.delete_subtask(),
                KeyCode::Char('u') => self.update_subtask(),
                KeyCode::Char('A') => self.start_adding_subtask(),
                KeyCode::Esc => self.cancel_subtask(),
                _ => {}
            },
            CurrentScreen::AddSubtask => match key_event.code {
                KeyCode::Enter => self.add_subtask(),
                KeyCode::Esc => self.cancel_adding_subtask(),
                KeyCode::Char(c) => self.handle_editing_input(c),
                KeyCode::Backspace => self.handle_backspace(),
                _ => {}
            },
        }
        Ok(())
    }

    fn select_next(&mut self) {
        self.todo_list.state.select_next();
        self.create_subtask_list();
    }

    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
        self.create_subtask_list();
    }

    fn create_subtask_list(&mut self) {
        // Clear the subtask_list and populate the subtask_list with the subtasks of the selected item,
        // if there are any. Otherwise, just clear the subtask_list
        self.subtask_list = SubtaskList {
            state: ListState::default(),
            items: Vec::new(),
        };

        let i = match self.todo_list.state.selected() {
            Some(i) => i,
            None => return,
        };

        if i < self.todo_list.items.len() {
            let subtasks = self.todo_list.items[i].subtasks.clone();
            self.subtask_list = SubtaskList {
                state: ListState::default(),
                items: subtasks,
            };
        }
    }

    fn select_next_subtask(&mut self) {
        self.subtask_list.state.select_next();
    }

    fn select_previous_subtask(&mut self) {
        self.subtask_list.state.select_previous();
    }

    fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].update_status();
        }
    }

    fn exit(&mut self) {
        self.save_todo_list();
        self.should_exit = true;
    }

    fn save_todo_list(&self) {
        // Generate the data from the list of tasks
        let mut data = String::new();
        let mut sections = HashMap::<String, Vec<Task>>::new();

        for section in &self.sections_order {
            if !sections.contains_key(section) {
                sections.insert(section.to_string(), vec![]);
            }
        }

        for task in &self.todo_list.items {
            let section = match task.status {
                Status::Todo => "## Todo",
                Status::Doing => "## Doing",
                Status::Done => "## Done",
            };
            sections.get_mut(section).unwrap().push(task.clone());
        }

        for section in &self.sections_order {
            data.push_str(&format!("{}\n", section));
            for task in sections.get(section).unwrap() {
                data.push_str(&format!("- {}\n", task.name));
                if !task.description.is_empty() {
                    data.push_str(&format!("    > {}\n", task.description));
                }
                if !task.subtasks.is_empty() {
                    for subtask in &task.subtasks {
                        if subtask.status {
                            data.push_str(&format!("    * [x] {}\n", subtask.name));
                        } else {
                            data.push_str(&format!("    * [ ] {}\n", subtask.name));
                        }
                    }
                }
            }
            data.push_str("\n");
        }

        fs::write(self.file_path.clone(), data).expect("Unable to write file");
    }

    fn start_editing(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.name_input = self.todo_list.items[i].name.clone();
            self.description_input = self.todo_list.items[i].description.clone();
            self.current_screen = CurrentScreen::Editing;
            self.currently_editing = Some(CurrentlyEditing::Name);
        }
    }

    fn save_edited_task(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            let task = &mut self.todo_list.items[i];
            task.name = self.name_input.clone();
            task.description = self.description_input.clone();
        }
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    fn cancel_editing(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    fn handle_editing_input(&mut self, c: char) {
        if let Some(CurrentlyEditing::Name) = self.currently_editing {
            self.name_input.push(c);
        } else if let Some(CurrentlyEditing::Description) = self.currently_editing {
            self.description_input.push(c);
        }
    }

    fn handle_backspace(&mut self) {
        if let Some(CurrentlyEditing::Name) = self.currently_editing {
            self.name_input.pop();
        } else if let Some(CurrentlyEditing::Description) = self.currently_editing {
            self.description_input.pop();
        }
    }

    fn toggle_editing_field(&mut self) {
        match self.currently_editing {
            Some(CurrentlyEditing::Name) => {
                self.currently_editing = Some(CurrentlyEditing::Description)
            }
            Some(CurrentlyEditing::Description) => {
                self.currently_editing = Some(CurrentlyEditing::Name)
            }
            None => self.currently_editing = Some(CurrentlyEditing::Name),
        }
    }

    fn start_adding(&mut self) {
        self.name_input = String::new();
        self.description_input = String::new();
        self.current_screen = CurrentScreen::AddTask;
        self.currently_editing = Some(CurrentlyEditing::Name);
    }

    fn add_new_task(&mut self) {
        self.todo_list.items.push(Task::new(
            self.name_input.clone(),
            self.description_input.clone(),
            Status::Todo,
            None,
        ));
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    fn cancel_adding(&mut self) {
        self.current_screen = CurrentScreen::Main;
        self.currently_editing = None;
    }

    fn start_deleting(&mut self) {
        self.current_screen = CurrentScreen::Deleting;
    }

    fn delete_task(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items.remove(i);
        }
        self.current_screen = CurrentScreen::Main;
    }

    fn cancel_deleting(&mut self) {
        self.current_screen = CurrentScreen::Main;
    }

    fn start_subtask(&mut self) {
        self.current_screen = CurrentScreen::Subtask;
    }

    fn cancel_subtask(&mut self) {
        self.current_screen = CurrentScreen::Main;
    }

    fn delete_subtask(&mut self) {
        if let Some(i) = self.subtask_list.state.selected() {
            self.todo_list.items[self.todo_list.state.selected().unwrap()]
                .subtasks
                .remove(i);

            self.subtask_list.items.remove(i);
        }
    }

    fn update_subtask(&mut self) {
        if let Some(i) = self.subtask_list.state.selected() {
            self.todo_list.items[self.todo_list.state.selected().unwrap()]
                .subtasks
                .get_mut(i)
                .unwrap()
                .update_status();

            self.subtask_list.items[i].update_status();
        }
    }

    fn start_adding_subtask(&mut self) {
        self.name_input = String::new();
        self.description_input = String::new();
        self.current_screen = CurrentScreen::AddSubtask;
        self.currently_editing = Some(CurrentlyEditing::Name);
    }

    fn cancel_adding_subtask(&mut self) {
        self.current_screen = CurrentScreen::Subtask;
        self.currently_editing = None;
    }

    fn add_subtask(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list
                .items
                .get_mut(i)
                .unwrap()
                .subtasks
                .push(Subtask::new(self.name_input.clone()));
        }

        self.subtask_list
            .items
            .push(Subtask::new(self.name_input.clone()));
        self.current_screen = CurrentScreen::Subtask;
        self.currently_editing = None;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [item_area, info_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        let [list_area, subtask_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(item_area);

        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(info_area, buf);
        self.render_subtasks(subtask_area, buf);
    }
}

// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Horme").bold().centered().render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new(
            "Press j/k to select, a to add new task, Enter to edit, d to delete, q to quit",
        )
        .centered()
        .render(area, buf);
    }

    // Iterate through the list of tasks and render them
    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Tasks").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(self.palette.background);

        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, todo_item)| {
                if i < self.todo_list.items.len() {
                    let bg_color = self.alternate_colors(i);
                    let fg_color = match todo_item.status {
                        Status::Todo => self.palette.light_yellow,
                        Status::Doing => self.palette.light_magenta,
                        Status::Done => self.palette.light_green,
                    };
                    Some(
                        ListItem::from(todo_item.name.clone())
                            .bg(bg_color)
                            .fg(fg_color),
                    )
                } else {
                    None
                }
            })
            .collect();

        match self.current_screen {
            CurrentScreen::Main => {
                let highlighted_style = if let Some(i) = self.todo_list.state.selected() {
                    if i < self.todo_list.items.len() {
                        match self.todo_list.items[i].status {
                            Status::Todo => Style::default()
                                .fg(self.palette.light_yellow)
                                .add_modifier(Modifier::BOLD),
                            Status::Doing => Style::default()
                                .fg(self.palette.light_magenta)
                                .add_modifier(Modifier::BOLD),
                            Status::Done => Style::default()
                                .fg(self.palette.light_green)
                                .add_modifier(Modifier::BOLD),
                        }
                    } else {
                        Style::default()
                    }
                } else {
                    Style::default()
                };

                // Create a list from all list items and highlight the currently selected one
                let list = List::new(items)
                    .block(block)
                    .highlight_style(highlighted_style)
                    .highlight_symbol(">> ")
                    .highlight_spacing(HighlightSpacing::Always);

                // Disambiguate this trait method as both `Widget` and `StatefulWidget`
                // share the `render` method
                StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
            }
            _ => {
                // If on other screens, just render the list of tasks,
                // without the possibility to select one
                let list = List::new(items).block(block);
                Widget::render(list, area, buf);
            }
        }
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // Show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(INFO_HEADER_STYLE)
            .bg(self.palette.black)
            .padding(Padding::horizontal(1));

        // Check if the user is editing an item
        match self.current_screen {
            CurrentScreen::Editing => {
                let input = match self.currently_editing {
                    Some(CurrentlyEditing::Name) => format!("Name: {} ", self.name_input),
                    Some(CurrentlyEditing::Description) => {
                        format!("Description: {}", self.description_input)
                    }
                    None => "".to_string(),
                };
                // Render the item info
                Paragraph::new(input)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            CurrentScreen::AddTask => {
                let input = match self.currently_editing {
                    Some(CurrentlyEditing::Name) => format!("Name: {} ", self.name_input),
                    Some(CurrentlyEditing::Description) => {
                        format!("Description: {}", self.description_input)
                    }
                    None => "".to_string(),
                };
                // Render the item info
                Paragraph::new(input)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            CurrentScreen::Deleting => {
                let input = format!(
                    "Are you sure you want to delete \"{}\"?\n
                        Press d to confirm, Esc to cancel",
                    self.todo_list.items[self.todo_list.state.selected().unwrap()].name
                );
                // Render the item info
                Paragraph::new(input)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            CurrentScreen::Subtask => {
                let info = format!(
                    "Modify subtask: {}",
                    if let Some(i) = self.subtask_list.state.selected() {
                        if i < self.subtask_list.items.len() {
                            self.subtask_list.items[i].name.clone()
                        } else {
                            self.subtask_list.items[i - 1].name.clone()
                        }
                    } else {
                        "No subtask selected".to_string()
                    }
                );
                // Render the item info
                Paragraph::new(info)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            _ => {
                let status_info = if let Some(i) = self.todo_list.state.selected() {
                    match self.todo_list.items[i].status {
                        Status::Todo => format!(
                            "◇ TODO: {}\n{}",
                            self.todo_list.items[i].name, self.todo_list.items[i].description
                        ),
                        Status::Doing => format!(
                            "◎ IN PROGRESS: {}\n{}",
                            self.todo_list.items[i].name, self.todo_list.items[i].description
                        ),
                        Status::Done => format!(
                            "✓ DONE: {}\n{}",
                            self.todo_list.items[i].name, self.todo_list.items[i].description
                        ),
                    }
                } else {
                    "No task selected".to_string()
                };
                let info = format!("{}", status_info);
                // Render the item info
                Paragraph::new(info)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
        }
    }

    fn render_subtasks(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(SUBTASK_HEADER_STYLE)
            .title(Line::raw("Subtasks").centered())
            .bg(self.palette.black);

        match self.current_screen {
            CurrentScreen::Subtask => {
                let subtasks: Vec<ListItem> = self
                    .subtask_list
                    .items
                    .iter()
                    .map(|subtask| ListItem::from(subtask))
                    .collect();

                let list = List::new(subtasks)
                    .block(block)
                    .highlight_symbol(">")
                    .highlight_spacing(HighlightSpacing::Always);
                // Disambiguate this trait method as both `Widget` and `StatefulWidget`
                // share the `render` method
                StatefulWidget::render(list, area, buf, &mut self.subtask_list.state);
            }
            CurrentScreen::AddSubtask => {
                let input = format!("Name: {}", self.name_input);
                // Render the item info
                Paragraph::new(input)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            _ => {
                let subtasks: String = self
                    .subtask_list
                    .items
                    .iter()
                    .map(|subtask| {
                        if subtask.status {
                            format!("[x] {}", subtask.name)
                        } else {
                            format!("[ ] {}", subtask.name)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                let info = format!("{}", subtasks);
                // Render the item info
                Paragraph::new(info)
                    .block(block)
                    .fg(self.palette.foreground)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
        }
    }
    const fn alternate_colors(&self, i: usize) -> Color {
        if i % 2 == 0 {
            self.palette.background
        } else {
            self.palette.black
        }
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!("◇ {}", value.name), TEXT_FG_COLOR),
            Status::Doing => Line::styled(format!("◎ {}", value.name), TEXT_FG_COLOR),
            Status::Done => Line::styled(format!("✓ {}", value.name), TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}

impl From<&Subtask> for ListItem<'_> {
    fn from(value: &Subtask) -> Self {
        let line = match value.status {
            true => Line::styled(format!("[x] {}", value.name), TEXT_FG_COLOR),
            false => Line::styled(format!("[ ] {}", value.name), TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
