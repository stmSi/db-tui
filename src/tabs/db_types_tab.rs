use std::io::{self, Result};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{block::*, *},
};

use super::DBTab;

#[derive(Debug)]
pub struct DbTypesTab {
    pub title: String,
    pub selected: usize,
    pub list_state: ListState,
    pub num_types: usize,
    pub disabled: bool,
}

pub enum DBTypes {
    POSTGRES,
    MYSQL,
    MARIA,
    SQLITE,
}

pub static POSTGRES_STR: &str = "PostgreSQL";
pub static MYSQL_STR: &str = "MySQL";
pub static MARIA_STR: &str = "MariaDB";
pub static SQLITE_STR: &str = "SQLite";

impl DBTypes {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DBTypes::POSTGRES => POSTGRES_STR,
            DBTypes::MYSQL => MYSQL_STR,
            DBTypes::MARIA => MARIA_STR,
            DBTypes::SQLITE => SQLITE_STR,
        }
    }
}

impl Default for DbTypesTab {
    fn default() -> Self {
        Self {
            title: "Types".to_string(),
            selected: 0,
            list_state: ListState::default().with_selected(Some(0)),
            num_types: 0,
            disabled: false,
        }
    }
}

impl DBTab for DbTypesTab {
    fn draw(&mut self, f: &mut Frame, rect: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(0)])
            .split(rect);

        let items = vec![
            ListItem::new(DBTypes::POSTGRES.as_str()),
            ListItem::new(DBTypes::MYSQL.as_str()),
            ListItem::new(DBTypes::MARIA.as_str()),
            ListItem::new(DBTypes::SQLITE.as_str()),
        ];
        self.num_types = items.len();

        let items = List::new(items)
            .block(Block::default().borders(Borders::NONE))
            .highlight_style(Color::Yellow)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol(">> ");

        f.render_stateful_widget(items, chunks[1], &mut self.list_state);

        Ok(())
    }

    fn handle_input(&mut self, key: KeyEvent) -> io::Result<()> {
        // Handle inputs specific to the DbTypes tab
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let current_selected = self.list_state.selected().unwrap_or(0);

                if current_selected == self.num_types - 1 {
                    self.list_state.select(Some(0));
                } else {
                    self.list_state
                        .select(Some((current_selected + 1).min(self.num_types - 1)));
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let current_selected = self.list_state.selected().unwrap_or(0);

                if current_selected == 0 {
                    self.list_state.select(Some(self.num_types - 1));
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
