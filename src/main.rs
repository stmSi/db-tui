use std::{io, sync::mpsc};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use log::*;
use ratatui::{
    prelude::*,
    widgets::{block::*, *},
};
use tui_logger::*;
mod args;
mod tabs;
mod tui;
mod ui;

use crate::tabs::DBTab;
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

#[derive(Debug)]
pub enum AppEvent {
    DBTypeSelected { db_type: DBTypes },
    ConnectionDetailsSubmitted { connection_string: String },
}

pub struct App {
    title: String,
    do_quit: QuitState,
    theme: SharedTheme,
    tabs: Vec<Box<dyn DBTab>>,
    current_tab_index: usize,
    db_type: Option<DBTypes>,
    event_bus: mpsc::Sender<AppEvent>,
    show_logs_window: bool,
    tui_widget_state: TuiWidgetState,
}

impl App {
    pub fn new(sender: mpsc::Sender<AppEvent>) -> Self {
        Self {
            title: " Database Manager ".to_string(),
            do_quit: QuitState::None,
            theme: SharedTheme::default(),
            tabs: vec![
                Box::new(DbTypesTab::default()),
                Box::new(DbConnectionsTab::default()),
                Box::new(DbDatabasesTab::default()),
                Box::new(DbTablesTab::default()),
            ],
            current_tab_index: 0,
            db_type: None,
            event_bus: sender,
            show_logs_window: false,
            tui_widget_state: TuiWidgetState::new().set_default_display_level(LevelFilter::Debug),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut tui::Tui, rx: mpsc::Receiver<AppEvent>) -> io::Result<()> {
        while self.do_quit == QuitState::None {
            if self.show_logs_window {
                terminal.draw(|frame| {
                    let _ = self.draw_log_window(frame);
                })?;
            } else {
                terminal.draw(|frame| {
                    if let Err(e) = self.draw(frame) {
                        log::error!("failed to draw: {:?}", e);
                    }
                })?;
            }
            self.handle_events()?;

            self.check_event_loop(&rx);
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                if self.show_logs_window {
                    self.handle_key_events_log_window(key_event)
                } else {
                    self.handle_key_event_main_app(key_event)
                }
            }
            _ => Ok(()),
        }
    }

    fn handle_key_event_main_app(&mut self, key_event: KeyEvent) -> io::Result<()> {
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
            KeyCode::F(12) => self.show_logs_window = !self.show_logs_window,
            _ => {
                let current_tab = &mut self.tabs[self.current_tab_index];
                current_tab.handle_input(key_event, &self.event_bus)?;
            }
        }

        Ok(())
    }

    fn handle_key_events_log_window(&mut self, key_event: KeyEvent) -> io::Result<()> {
        match key_event.code {
            KeyCode::Char('j') => {
                self.tui_widget_state
                    .transition(TuiWidgetEvent::NextPageKey);
            }
            KeyCode::Char('k') => self
                .tui_widget_state
                .transition(TuiWidgetEvent::PrevPageKey),
            KeyCode::Char('q') => self.show_logs_window = false,
            KeyCode::F(12) => self.show_logs_window = !self.show_logs_window,
            _ => {}
        }

        Ok(())
    }

    fn switch_next_tab(&mut self) {
        let enabled_tabs_num = self.tabs.iter().filter(|tab| !tab.is_disabled()).count();
        if enabled_tabs_num < 2 {
            return; // dont switch if there is only one tab
        }
        self.current_tab_index = (self.current_tab_index + 1) % self.tabs.len();
        if self.tabs[self.current_tab_index].is_disabled() {
            // skip disabled tabs
            self.switch_next_tab();
        }
    }

    fn switch_prev_tab(&mut self) {
        let enabled_tabs_num = self.tabs.iter().filter(|tab| !tab.is_disabled()).count();
        if enabled_tabs_num < 2 {
            return; // dont switch if there is only one tab
        }

        self.current_tab_index = if self.current_tab_index == 0 {
            self.tabs.len() - 1
        } else {
            self.current_tab_index - 1
        };

        if self.tabs[self.current_tab_index].is_disabled() {
            // skip disabled tabs
            self.switch_prev_tab();
        }
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

        let divider = " | ";
        let tabs: Vec<Line> = self
            .tabs
            .iter()
            // .enumerate()
            .map(|tab| {
                let title = tab.get_title();
                let line = Line::from(title);
                line.style(self.theme.tab(!tab.is_disabled(), false))
            })
            .collect();

        f.render_widget(
            Tabs::new(tabs)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(self.theme.block(false)),
                )
                .highlight_style(self.theme.tab(false, true))
                .divider(divider)
                .select(self.current_tab_index),
            r,
        );
    }

    fn draw_log_window(&self, f: &mut Frame) -> io::Result<()> {
        let size = f.size();

        TuiLoggerWidget::default()
            .block(Block::bordered().title(" Logs ").borders(Borders::TOP))
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator('|')
            .output_timestamp(Some("%F %H:%M:%S%.3f".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(false)
            .output_file(true)
            .output_line(true)
            .state(&self.tui_widget_state)
            .render(size, f.buffer_mut());

        Ok(())
    }

    fn check_event_loop(&mut self, rx: &mpsc::Receiver<AppEvent>) {
        loop {
            let event_result = rx.try_recv();
            if let Ok(event) = event_result {
                match event {
                    AppEvent::DBTypeSelected { db_type } => {
                        // Handle database type selection
                        debug!("Database Type Selected: {:?}", db_type);
                        self.db_type = Some(db_type);
                        self.switch_next_tab();
                    }
                    AppEvent::ConnectionDetailsSubmitted { connection_string } => {
                        // Handle connection details submission TODO:
                        debug!("Connection String: {}", connection_string);
                    }
                }
            } else if event_result.is_err() {
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = init_logger(LevelFilter::Debug);
    set_default_level(LevelFilter::Debug);

    let (tx, rx) = mpsc::channel();
    let mut app = App::new(tx);
    debug!("Starting application");

    let mut terminal = tui::init()?;
    let app_result = app.run(&mut terminal, rx);
    tui::restore()?;
    app_result
}
