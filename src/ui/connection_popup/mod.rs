use std::{io, sync::mpsc};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use log::debug;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use tui_textarea::TextArea;

use crate::{App, AppEvent};

#[derive(Debug, PartialEq, Eq)]
enum ActiveText {
    Host,
    DbName,
    Username,
    Password,
}

#[derive(Debug)]
pub struct DbConnectionPopup<'a> {
    db_driver_name: &'a str,
    host_txt: TextArea<'a>,
    dbname_txt: TextArea<'a>,
    username_txt: TextArea<'a>,
    password_txt: TextArea<'a>,
    active_txt: ActiveText,
    host_err_msg: String,
    dbname_err_msg: String,
    username_err_msg: String,
    password_err_msg: String,
}

const FOOTER_TEXT: &str =
    "Enter or <Ctrl-m>: confirm | Esc or <Ctrl-c>: Cancel | Tab: Change focused control";
const FOOTER_MARGIN: u16 = 15;

impl<'a> DbConnectionPopup<'a> {
    pub fn new(db_driver_name: &'a str) -> Self {
        let mut password_area = TextArea::default();
        password_area.set_cursor_style(Style::default());
        password_area.set_mask_char('\u{2022}'); // U+2022 BULLET (•)

        Self {
            db_driver_name,
            host_txt: TextArea::default(),
            dbname_txt: TextArea::default(),
            username_txt: TextArea::default(),
            password_txt: password_area,
            active_txt: ActiveText::Host,
            host_err_msg: String::default(),
            dbname_err_msg: String::default(),
            username_err_msg: String::default(),
            password_err_msg: String::default(),
        }
    }

    pub fn get_db_driver_name(&self) -> &str {
        self.db_driver_name
    }

    pub fn render_widget(&mut self, frame: &mut Frame, area: Rect) {
        let mut area = centered_rect_exact_height(70, 20, area);

        const FOOTER_LEN: u16 = FOOTER_TEXT.len() as u16 + FOOTER_MARGIN;
        if area.width < FOOTER_LEN {
            area.height += FOOTER_LEN / area.width;
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("New DB Connection: {}", self.db_driver_name));

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(4)
            .vertical_margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(1),
                ]
                .as_ref(),
            )
            .split(area);

        self.update_cursor_styles();
        self.update_border_styles();

        self.render_text_area(frame, &self.host_txt, chunks[0], "Host", &self.host_err_msg);
        self.render_text_area(
            frame,
            &self.dbname_txt,
            chunks[1],
            "Database Name",
            &self.dbname_err_msg,
        );
        self.render_text_area(
            frame,
            &self.username_txt,
            chunks[2],
            "Username",
            &self.username_err_msg,
        );
        self.render_text_area(
            frame,
            &self.password_txt,
            chunks[3],
            "Password",
            &self.password_err_msg,
        );

        let footer = Paragraph::new(FOOTER_TEXT)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default()),
            );
        frame.render_widget(footer, chunks[4]);
    }

    fn update_cursor_styles(&mut self) {
        let active_style = Style::default().bg(Color::White).fg(Color::Black);
        let inactive_style = Style::default().bg(Color::Reset);

        self.host_txt.set_cursor_style(inactive_style);
        self.dbname_txt.set_cursor_style(inactive_style);
        self.username_txt.set_cursor_style(inactive_style);
        self.password_txt.set_cursor_style(inactive_style);

        debug!("Active text: {:?}", self.active_txt);
        match self.active_txt {
            ActiveText::Host => self.host_txt.set_cursor_style(active_style),
            ActiveText::DbName => self.dbname_txt.set_cursor_style(active_style),
            ActiveText::Username => self.username_txt.set_cursor_style(active_style),
            ActiveText::Password => self.password_txt.set_cursor_style(active_style),
        }
    }

    fn update_border_styles(&mut self) {
        let border_style = Block::default().borders(Borders::ALL);
        self.host_txt.set_block(border_style.clone().title("Host"));
        self.dbname_txt
            .set_block(border_style.clone().title("Database Name"));
        self.username_txt
            .set_block(border_style.clone().title("Username"));
        self.password_txt
            .set_block(border_style.clone().title("Password"));

        match self.active_txt {
            ActiveText::Host => self.host_txt.set_block(
                border_style
                    .style(Style::default().fg(Color::Red))
                    .title("Host"),
            ),
            ActiveText::DbName => self.dbname_txt.set_block(
                border_style
                    .style(Style::default().fg(Color::Red))
                    .title("Database Name"),
            ),
            ActiveText::Username => self.username_txt.set_block(
                border_style
                    .style(Style::default().fg(Color::Red))
                    .title("Username"),
            ),
            ActiveText::Password => self.password_txt.set_block(
                border_style
                    .style(Style::default().fg(Color::Red))
                    .title("Password"),
            ),
        }
    }

    fn render_text_area(
        &self,
        frame: &mut Frame,
        text_area: &TextArea,
        area: Rect,
        title: &str,
        error_msg: &str,
    ) {
        let style = if error_msg.is_empty() {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        frame.render_widget(text_area.widget(), area);
    }
    pub fn handle_input(
        &mut self,
        key_event: &KeyEvent,
        app_event_bus: &mpsc::Sender<AppEvent>,
    ) -> io::Result<()> {
        let has_ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);

        match key_event.code {
            KeyCode::Esc => self.cancel(app_event_bus),
            KeyCode::Char('c') => {
                if has_ctrl {
                    self.cancel(app_event_bus)
                }
            }
            KeyCode::Enter => self.confirm()?,
            KeyCode::Tab | KeyCode::Down => self.navigate_to_next_field(),
            KeyCode::Up => self.navigate_to_previous_field(),
            _ => self.handle_text_input(key_event),
        }
        Ok(())
    }

    fn cancel(&mut self, app_event_bus: &mpsc::Sender<AppEvent>) {
        app_event_bus.send(AppEvent::CancelClosePopup).unwrap();
        // TODO: Handle cleanup or state reset before closing the popup
    }

    fn confirm(&mut self) -> io::Result<()> {
        if self.validate_all() {
            // Proceed with processing the confirmed data
            // TODO: update the database settings in the application context
        }
        Ok(())
    }

    fn navigate_to_next_field(&mut self) {
        self.active_txt = match self.active_txt {
            ActiveText::Host => ActiveText::DbName,
            ActiveText::DbName => ActiveText::Username,
            ActiveText::Username => ActiveText::Password,
            ActiveText::Password => ActiveText::Host,
        };
    }

    fn navigate_to_previous_field(&mut self) {
        self.active_txt = match self.active_txt {
            ActiveText::Host => ActiveText::Password,
            ActiveText::DbName => ActiveText::Host,
            ActiveText::Username => ActiveText::DbName,
            ActiveText::Password => ActiveText::Username,
        };
    }

    fn handle_text_input(&mut self, input: &KeyEvent) {
        let text_area = match self.active_txt {
            ActiveText::Host => &mut self.host_txt,
            ActiveText::DbName => &mut self.dbname_txt,
            ActiveText::Username => &mut self.username_txt,
            ActiveText::Password => &mut self.password_txt,
        };
        text_area.input(*input);
        self.validate_active_field();
    }

    fn validate_all(&mut self) -> bool {
        self.validate_host();
        self.validate_dbname();
        self.validate_username();
        self.validate_password();

        self.host_err_msg.is_empty()
            && self.dbname_err_msg.is_empty()
            && self.username_err_msg.is_empty()
            && self.password_err_msg.is_empty()
    }

    fn validate_host(&mut self) {
        if self.host_txt.lines()[0].is_empty() {
            self.host_err_msg = "Host cannot be empty.".into();
        } else {
            self.host_err_msg.clear();
        }
    }

    fn validate_dbname(&mut self) {
        if self.dbname_txt.lines()[0].is_empty() {
            self.dbname_err_msg = "Database name cannot be empty.".into();
        } else {
            self.dbname_err_msg.clear();
        }
    }

    fn validate_username(&mut self) {
        if self.username_txt.lines()[0].is_empty() {
            self.username_err_msg = "Username cannot be empty.".into();
        } else {
            self.username_err_msg.clear();
        }
    }

    fn validate_password(&mut self) {
        if self.password_txt.lines()[0].is_empty() {
            self.password_err_msg = "Password cannot be empty.".into();
        } else {
            self.password_err_msg.clear();
        }
    }

    fn validate_active_field(&mut self) {
        match self.active_txt {
            ActiveText::Host => self.validate_host(),
            ActiveText::DbName => self.validate_dbname(),
            ActiveText::Username => self.validate_username(),
            ActiveText::Password => self.validate_password(),
        }
    }
}

fn centered_rect_exact_height(percent_x: u16, height: u16, area: Rect) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = height;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}
