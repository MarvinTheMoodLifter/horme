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

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

use crate::task::Status;
use crate::task::Task;

#[derive(Debug)]
pub struct App {
    todo_list: TodoList,
    pub should_exit: bool,
}

#[derive(Debug, Default)]
pub struct TodoList {
    state: ListState,
    items: Vec<Task>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            todo_list: TodoList::from_iter([
                ("Rewrite everything with Rust!", "I can't hold my inner voice. He tells me to rewrite the complete universe with Rust", Status::Todo),
                ("Rewrite all of your tui apps with Ratatui", "Yes, you heard that right. Go and replace your tui with Ratatui.", Status::Done),
                ("Pet your cat", "Minnak loves to be pet by you! Don't forget to pet and give some treats!", Status::Todo),
                ("Walk with your dog", "Max is bored, go walk with him!", Status::Todo),
                ("Pay the bills", "Pay the train subscription!!!", Status::Done),
                ("Refactor list example", "If you see this info that means I completed this task!", Status::Done),
            ]),
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
    pub fn new(todo_list: Vec<Task>) -> Self {
        let state = ListState::default();

        Self {
            todo_list: TodoList {
                state,
                items: todo_list,
            },
            should_exit: false,
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
        match key_event.code {
            KeyCode::Char('j') => self.select_next(),
            KeyCode::Char('k') => self.select_previous(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
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
            self.todo_list.items[i].update_status(Status::Done);
        }
    }

    pub fn exit(&mut self) {
        self.should_exit = true;
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
        // Get the info depending on the item's state
        let info = if let Some(i) = self.todo_list.state.selected() {
            match self.todo_list.items[i].status {
                Status::Todo => format!("◇ TODO: {}", self.todo_list.items[i].name),
                Status::Doing => format!("◎ IN PROGRESS: {}", self.todo_list.items[i].name),
                Status::Done => format!("✓ DONE: {}", self.todo_list.items[i].name),
            }
        } else {
            "No task selected".to_string()
        };

        // Show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("TODO Info").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        // Render the item info
        Paragraph::new(info)
            .block(block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(area, buf);
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
