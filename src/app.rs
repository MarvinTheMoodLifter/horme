use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{
        palette::tailwind::{BLUE, GREEN, SLATE},
        Color, Modifier, Style, Stylize,
    },
    symbols,
    terminal::Terminal,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget, Wrap,
    },
};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const TEXT_FG_EDITING: Color = BLUE.c400;
const TEXT_FG_ADDING: Color = GREEN.c400;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

use crate::task::Status;
use crate::task::Task;

//#[derive(Debug)]
pub struct App {
    pub name_input: String,
    pub description_input: String,
    pub todo_list: TodoList,
    pub file_path: PathBuf,
    pub sections_order: Vec<String>,
    pub should_exit: bool,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

#[derive(Debug, Default)]
pub struct TodoList {
    pub state: ListState,
    pub items: Vec<Task>,
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    Editing,
    Adding,
    Deleting,
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
            file_path: file_path.to_path_buf(),
            sections_order: vec![
                "## Todo".to_string(),
                "## Doing".to_string(),
                "## Done".to_string(),
            ],
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
            CurrentScreen::Adding => match key_event.code {
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
        }
        Ok(())
    }

    pub fn select_none(&mut self) {
        self.todo_list.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.todo_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.todo_list.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.todo_list.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.todo_list.state.select_last();
    }

    pub fn toggle_status(&mut self) {
        if let Some(i) = self.todo_list.state.selected() {
            self.todo_list.items[i].update_status();
        }
    }

    pub fn exit(&mut self) {
        self.save_todo_list();
        self.should_exit = true;
    }

    pub fn save_todo_list(&self) {
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
        self.current_screen = CurrentScreen::Adding;
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
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);
        App::render_header(header_area, buf);
        App::render_footer(footer_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(item_area, buf);
    }
}

// Rendering logic for the app
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Horme").bold().centered().render(area, buf);
    }

    fn render_footer(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Press j/k to select, q to quit")
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
            .bg(NORMAL_ROW_BG);

        let items: Vec<ListItem> = self
            .todo_list
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item.name.clone()).bg(color)
            })
            .collect();

        // Create a list from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">> ")
            .highlight_spacing(HighlightSpacing::Always);

        // Disambiguate this trait method as both `Widget` and `StatefulWidget`
        // share the `render` method
        StatefulWidget::render(list, area, buf, &mut self.todo_list.state);
    }

    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // Show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
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
                    .fg(TEXT_FG_EDITING)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            CurrentScreen::Adding => {
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
                    .fg(TEXT_FG_ADDING)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            CurrentScreen::Deleting => {
                let input = format!(
                    "Are you sure you want to delete <{}>?\n
                        Press d to confirm, Esc to cancel",
                    self.name_input
                );
                // Render the item info
                Paragraph::new(input)
                    .block(block)
                    .fg(TEXT_FG_EDITING)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
            _ => {
                let info = if let Some(i) = self.todo_list.state.selected() {
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
                // Render the item info
                Paragraph::new(info)
                    .block(block)
                    .fg(TEXT_FG_COLOR)
                    .wrap(Wrap { trim: false })
                    .render(area, buf);
            }
        }
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.status {
            Status::Todo => Line::styled(format!("◇ {}", value.name), TEXT_FG_COLOR),
            Status::Doing => Line::styled(format!("◎ {}", value.name), TEXT_FG_COLOR),
            Status::Done => Line::styled(format!("✓ {}", value.name), COMPLETED_TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
