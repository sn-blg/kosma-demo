use std::error;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    dbg: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            dbg: String::from("none"),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn on_left_mouse_up(&mut self, column: u16, row: u16) {
        self.dbg = format!("mouse - {} : {}", column, row);
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            .split(frame.size());

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(chunks[1]);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.dbg.as_ref());
        frame.render_widget(block, right_chunks[0]);

        let block = Block::default().borders(Borders::ALL);
        frame.render_widget(block, right_chunks[1]);

        App::menu(frame, chunks[0]);
    }

    fn menu<B: Backend>(f: &mut Frame<B>, area: Rect) {
        let mut items: Vec<ListItem> = Vec::new();

        items.push(ListItem::new(vec![Spans::from(vec![Span::raw("Item1")])]));
        items.push(ListItem::new(vec![Spans::from(vec![Span::raw("Item2")])]));
        items.push(ListItem::new(vec![Spans::from(vec![Span::raw("Item3")])]));
        items.push(ListItem::new(vec![Spans::from(vec![Span::raw("Item4")])]));

        let items = List::new(items).block(Block::default().borders(Borders::ALL).title("menu"));

        f.render_widget(items, area);
    }
}
