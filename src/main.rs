use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use unicode_width::UnicodeWidthStr;
mod args;
mod keys;
mod strings;
mod tabs;
mod tui;
mod ui;

use crate::tabs::DBTab;
use keys::{KeyConfig, SharedKeyConfig};
use tabs::db_connecitons_tab::*;
use tabs::db_databases_tab::*;
use tabs::db_tables_tab::*;
use tabs::db_types_tab::*;

use ui::style::SharedTheme;
#[derive(Clone, Debug, PartialEq)]
pub enum QuitState {
    None,
    Close,
}

// #[derive(Debug)]
pub struct App {
    title: String,
    do_quit: QuitState,
    key_config: SharedKeyConfig,
    theme: SharedTheme,
    tabs: Vec<Box<dyn DBTab>>,
    current_tab_index: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: " Database Manager ".to_string(),
            do_quit: QuitState::None,
            key_config: SharedKeyConfig::default(),
            theme: SharedTheme::default(),
            tabs: vec![
                Box::new(DbTypesTab::default()),
                Box::new(DbConnectionsTab::default()),
                Box::new(DbDatabasesTab::default()),
                Box::new(DbTablesTab::default()),
            ],
            current_tab_index: 0,
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while self.do_quit == QuitState::None {
            terminal.draw(|frame| {
                if let Err(e) = self.draw(frame) {
                    log::error!("failed to draw: {:?}", e);
                }
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('q') => self.quit(),
            KeyCode::Tab => {
                if key_event.modifiers.is_empty() {
                    self.switch_next_tab()
                }
                if key_event.modifiers.contains(event::KeyModifiers::SHIFT) {
                    self.switch_prev_tab()
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {}
            _ => {}
        }

        let _ = self.tabs[self.current_tab_index].handle_input(key_event);

        Ok(())
    }

    fn switch_next_tab(&mut self) {
        self.current_tab_index = (self.current_tab_index + 1) % self.tabs.len();
    }

    fn switch_prev_tab(&mut self) {
        self.current_tab_index = if self.current_tab_index == 0 {
            self.tabs.len() - 1
        } else {
            self.current_tab_index - 1
        };
    }

    fn quit(&mut self) {
        self.do_quit = QuitState::Close;
    }

    fn draw(&mut self, frame: &mut Frame) -> io::Result<()> {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1), // title
                Constraint::Length(3), // top bar tabs list
                Constraint::Min(5),    // main chunk for selected tab
                Constraint::Length(1), // bottom
            ],
        )
        .split(frame.size());
        let title_chunk = main_layout[0];
        let top_bar_chunk = main_layout[1];
        let main_body_chunk = main_layout[2];
        let bottom_chunk = main_layout[3];
        // let [title_chunk, tab_bar_chunk, body_chunk, bottom_chunk] = main_layout;
        self.draw_title(frame, title_chunk);
        self.draw_top_bar(frame, top_bar_chunk);

        // draw current tab index
        let _ = self.tabs[self.current_tab_index].draw(frame, main_body_chunk);

        frame.render_widget(
            Paragraph::new(self.current_tab_index.to_string()),
            bottom_chunk,
        );
        Ok(())
    }

    fn draw_title(&self, f: &mut Frame, r: Rect) {
        let title = Title::from(self.title.clone()).alignment(Alignment::Center);
        f.render_widget(Block::default().borders(Borders::TOP).title(title), r);
    }

    fn draw_top_bar(&self, f: &mut Frame, r: Rect) {
        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tab_labels: Vec<Span> = self
            .tabs
            .iter()
            .map(|tab| Span::raw(tab.get_title()))
            .collect();

        let divider = strings::tab_divider(&self.key_config);

        let tabs: Vec<Line> = tab_labels.into_iter().map(Line::from).collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.block(false)),
                )
                .style(self.theme.tab(false))
                .highlight_style(self.theme.tab(true))
                .divider(divider)
                .select(self.current_tab_index),
            r,
        );
    }
}

fn main() -> io::Result<()> {
    let mut terminal = tui::init()?;
    let key_config = KeyConfig::init()
        .map_err(|e| eprintln!("KeyConfig loading error: {e}"))
        .unwrap_or_default();

    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
