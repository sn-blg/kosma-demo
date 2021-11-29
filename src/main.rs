use crossterm::event::{self, Event, KeyCode};
use kosma_demo::app::{App, AppResult};
use kosma_demo::tui::Tui;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.init()?;

    loop {
        tui.draw(&mut app)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    tui.exit()?;
    Ok(())
}
