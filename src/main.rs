use kosma_demo::app::{App, AppResult};
use kosma_demo::event::{Event, EventHandler};
use kosma_demo::handler;
use kosma_demo::tui::Tui;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handler::handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handler::handle_mouse_events(mouse_event, &mut app)?,
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
