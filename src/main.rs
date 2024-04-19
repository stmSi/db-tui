use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
mod args;
mod keys;
mod strings;
mod tui;
mod ui;

use keys::{KeyConfig, SharedKeyConfig};
use ui::style::SharedTheme;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Debug, PartialEq)]
pub enum QuitState {
    None,
    Close,
}

#[derive(Debug)]
pub struct App {
    title: String,
    do_quit: QuitState,
    key_config: SharedKeyConfig,
    tab: usize,
    tabs_size: usize,
    theme: SharedTheme,
}

impl Default for App {
    fn default() -> Self {
        Self {
            title: "Database Manager".to_string(),
            do_quit: QuitState::None,
            key_config: SharedKeyConfig::default(),
            tab: 0,
            tabs_size: 3,
            theme: SharedTheme::default(),
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
            _ => {}
        }

        Ok(())
    }

    fn switch_next_tab(&mut self) {
        self.tab = (self.tab + 1) % self.tabs_size;
    }

    fn switch_prev_tab(&mut self) {
        self.tab = if self.tab == 0 {
            self.tabs_size - 1
        } else {
            self.tab - 1
        };
    }

    fn quit(&mut self) {
        self.do_quit = QuitState::Close;
    }

    fn draw(&self, frame: &mut Frame) -> io::Result<()> {
        let main_layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(1), // title
                Constraint::Length(1), // top bar tabs list
                Constraint::Min(1),    // main chunk for selected tab
                Constraint::Length(5), // bottom
            ],
        )
        .split(frame.size());
        // let [title_chunk, tab_bar_chunk, body_chunk, bottom_chunk] = main_layout;
        self.draw_title(frame, main_layout[0]);
        self.draw_top_bar(frame, main_layout[1]);

        // draw current tab index
        frame.render_widget(Paragraph::new(self.tab.to_string()), main_layout[2]);
        Ok(())
    }

    fn draw_title(&self, f: &mut Frame, r: Rect) {
        let title = Title::from(self.title.clone()).alignment(Alignment::Center);
        f.render_widget(Block::default().borders(Borders::TOP).title(title), r);
    }

    //TODO: make this dynamic
    fn draw_top_bar(&self, f: &mut Frame, r: Rect) {
        const DIVIDER_PAD_SPACES: usize = 2;
        const SIDE_PADS: usize = 2;
        const MARGIN_LEFT_AND_RIGHT: usize = 2;

        let r = r.inner(&Margin {
            vertical: 0,
            horizontal: 1,
        });

        let tab_labels = [
            Span::raw(strings::tab_db_types(&self.key_config)),
            Span::raw(strings::tab_db_connections(&self.key_config)),
            Span::raw(strings::tab_db_tables_and_schema(&self.key_config)),
        ];
        let divider = strings::tab_divider(&self.key_config);

        // heuristic, since tui doesn't provide a way to know
        // how much space is needed to draw a `Tabs`
        let tabs_len: usize = tab_labels.iter().map(Span::width).sum::<usize>()
            + tab_labels.len().saturating_sub(1) * (divider.width() + DIVIDER_PAD_SPACES)
            + SIDE_PADS
            + MARGIN_LEFT_AND_RIGHT;

        let left_right = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Length(u16::try_from(tabs_len).unwrap_or(r.width)),
                Constraint::Min(0),
            ])
            .split(r);

        let table_area = r; // use entire area to allow drawing the horizontal separator line
        let text_area = left_right[1];

        let tabs: Vec<Line> = tab_labels.into_iter().map(Line::from).collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::LEFT)
                        .border_style(self.theme.block(false)),
                )
                .style(self.theme.tab(false))
                .highlight_style(self.theme.tab(true))
                .divider(divider)
                .select(self.tab),
            table_area,
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
