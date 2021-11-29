use crate::app::{App, AppResult};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::backend::Backend;
use tui::Terminal;

pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self { terminal }
    }

    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        self.terminal.draw(|frame| app.render(frame))?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

        self.terminal.show_cursor()?;
        Ok(())
    }
}
