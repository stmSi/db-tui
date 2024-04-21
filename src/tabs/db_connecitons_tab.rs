use std::{
    io::{self, Result},
    sync::mpsc,
};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{block::*, *},
};

use crate::{App, AppEvent};

use super::DBTab;

#[derive(Debug)]
pub struct DbConnectionsTab {
    pub title: String,
    pub connections: Vec<String>,
    pub selected: usize,
    pub list_state: ListState,
    pub num_tabs: usize,
    pub disabled: bool,
}

impl Default for DbConnectionsTab {
    fn default() -> Self {
        Self {
            title: "Connections".to_string(),
            connections: vec![],
            selected: 0,
            list_state: ListState::default().with_selected(Some(0)),
            num_tabs: 0,
            disabled: false,
        }
    }
}

impl DBTab for DbConnectionsTab {
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

    fn handle_input(
        &mut self,
        key: KeyEvent,
        app_event_bus: &mpsc::Sender<AppEvent>,
    ) -> io::Result<()> {
        // match key.code {
        //     KeyCode::Char('j') | KeyCode::Down => {
        //         self.list_state.move_down();
        //     }
        //     KeyCode::Char('k') | KeyCode::Up => {
        //         self.list_state.move_up();
        //     }
        //     _ => {}
        // }

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
