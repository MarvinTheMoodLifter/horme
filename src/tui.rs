use std::{io, io::stdout};

use color_eyre::config::HookBuilder;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    terminal::Terminal,
};

pub fn init_error_hooks() -> color_eyre::Result<()> {
    let (panic, error) = HookBuilder::default().into_hooks();
    let panic = panic.into_panic_hook();
    let error = error.into_eyre_hook();
    color_eyre::eyre::set_hook(Box::new(move |e| {
        let _ = restore_terminal();
        error(e)
    }))?;
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        panic(info);
    }));
    Ok(())
}

pub fn init_terminal() -> io::Result<Terminal<impl Backend>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

pub fn restore_terminal() -> io::Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()
}

//use std::io::{self, stdout, Stdout};
//
//use crossterm::{execute, terminal::*};
//use ratatui::prelude::*;
//
//// A type alias for the terminal type used in this application
//pub type Tui = Terminal<CrosstermBackend<Stdout>>;
//
//// Initialize the terminal
//#[allow(dead_code)]
//pub fn init() -> io::Result<Tui> {
//    execute!(stdout(), EnterAlternateScreen)?;
//    enable_raw_mode()?;
//    Terminal::new(CrosstermBackend::new(stdout()))
//}
//
//// Restore the terminal to its original state
//#[allow(dead_code)]
//pub fn restore() -> io::Result<()> {
//    execute!(stdout(), LeaveAlternateScreen)?;
//    disable_raw_mode()?;
//    Ok(())
//}
