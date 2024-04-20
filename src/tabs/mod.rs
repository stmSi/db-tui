use std::io;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub mod db_types_tab;

pub trait DBTab {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> io::Result<()>;
    fn handle_input(&mut self, key: KeyEvent) -> io::Result<()>;
}
