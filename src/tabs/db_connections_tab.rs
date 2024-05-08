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
    pub connections: Vec<Connection>,
    pub selected: usize,
    pub list_state: ListState,
    pub num_connections: usize,
    pub disabled: bool,
}

#[derive(Debug)]
pub struct Connection {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub is_create_new: bool,
}

impl Default for DbConnectionsTab {
    fn default() -> Self {
        let create_new_connection = Connection {
            name: "Create New Connection".to_string(),
            host: "".to_string(),
            user: "".to_string(),
            password: "".to_string(),
            port: 0,
            is_create_new: true,
        };

        Self {
            title: "Connections".to_string(),
            connections: vec![create_new_connection],
            selected: 0,
            list_state: ListState::default().with_selected(Some(0)),
            num_connections: 1,
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

        let items = self
            .connections
            .iter()
            .map(|conn| {
                ListItem::new(Span::styled(
                    conn.name.clone(),
                    Style::default().fg(Color::White),
                ))
            })
            .collect::<Vec<ListItem>>();

        self.num_connections = items.len();

        let items = List::new(items)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(Color::Yellow)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">> ");

        frame.render_stateful_widget(items, chunks[1], &mut self.list_state);

        Ok(())
    }

    fn handle_input(
        &mut self,
        key: KeyEvent,
        app_event_bus: &mpsc::Sender<AppEvent>,
    ) -> io::Result<()> {
        match key.code {
            KeyCode::Enter => {
                app_event_bus.send(AppEvent::NewConnection).unwrap();
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let current_selected = self.list_state.selected().unwrap_or(0);

                if current_selected == self.num_connections - 1 {
                    self.list_state.select(Some(0));
                } else {
                    self.list_state
                        .select(Some((current_selected + 1).min(self.num_connections - 1)));
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let current_selected = self.list_state.selected().unwrap_or(0);

                if current_selected == 0 {
                    self.list_state.select(Some(self.num_connections - 1));
                } else {
                    self.list_state.select(Some(current_selected - 1));
                }
            }
            _ => {}
        }
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
