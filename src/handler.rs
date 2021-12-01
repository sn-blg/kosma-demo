use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        KeyCode::Esc => {
            app.running = false;
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.running = false;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(event: MouseEvent, app: &mut App) -> AppResult<()> {
    match event.kind {
        MouseEventKind::Down(button) => {
            if let MouseButton::Left = button {
                app.on_left_mouse_up(event.column, event.row);
            }
        }
        _ => {}
    }
    Ok(())
}
