use std::{io, sync::mpsc};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::AppEvent;

const FOOTER_TEXT: &str = "Q or Enter: quit | Esc or C: Cancel";
const FOOTER_MARGIN: u16 = 15;

#[derive(Debug)]
pub struct QuitConfirmPopup;

impl QuitConfirmPopup {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_widget(&mut self, frame: &mut Frame, area: Rect) {
        let mut area = centered_rect_exact_height(40, 8, area);

        const FOOTER_LEN: u16 = FOOTER_TEXT.len() as u16 + FOOTER_MARGIN;
        if area.width < FOOTER_LEN {
            area.height += FOOTER_LEN / area.width;
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Quit Confirmation");

        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(4)
            .vertical_margin(2)
            .constraints([Constraint::Length(3), Constraint::Length(3)].as_ref())
            .split(area);

        let confirm_text = Paragraph::new("Are you sure you want to quit?")
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default()),
            );
        frame.render_widget(confirm_text, chunks[0]);

        let footer = Paragraph::new(FOOTER_TEXT)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false })
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .style(Style::default()),
            );
        frame.render_widget(footer, chunks[1]);
    }

    pub fn handle_input(
        &mut self,
        key_event: &KeyEvent,
        app_event_bus: &mpsc::Sender<AppEvent>,
    ) -> io::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('c') => self.cancel_close(app_event_bus),
            KeyCode::Enter | KeyCode::Char('q') => self.confirm_quit(app_event_bus),
            _ => {}
        }
        Ok(())
    }

    fn cancel_close(&mut self, app_event_bus: &mpsc::Sender<AppEvent>) {
        app_event_bus.send(AppEvent::CancelClosePopup).unwrap();
    }

    fn confirm_quit(&mut self, app_event_bus: &mpsc::Sender<AppEvent>) {
        app_event_bus.send(AppEvent::ConfirmQuitApp).unwrap();
    }
}

fn centered_rect_exact_height(percent_x: u16, height: u16, area: Rect) -> Rect {
    let popup_width = area.width * percent_x / 100;
    let popup_height = height;
    let popup_x = (area.width - popup_width) / 2;
    let popup_y = (area.height - popup_height) / 2;
    Rect::new(popup_x, popup_y, popup_width, popup_height)
}
