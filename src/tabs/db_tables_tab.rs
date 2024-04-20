use std::io::{self, Result};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{block::*, *},
};

use super::DBTab;

#[derive(Debug)]
pub struct DbTablesTab {
    pub title: String,
    pub disabled: bool,
}

impl Default for DbTablesTab {
    fn default() -> Self {
        Self {
            title: "Tables".to_string(),
            disabled: true,
        }
    }
}

impl DBTab for DbTablesTab {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> io::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        Paragraph::new(Span::styled(
            self.get_title(),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ))
        .render(chunks[0], frame.buffer_mut());

        Ok(())
    }

    fn handle_input(&mut self, key: KeyEvent) -> io::Result<()> {
        Ok(())
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }
}
