use std::io;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

pub mod db_connecitons_tab;
pub mod db_databases_tab;
pub mod db_tables_tab;
pub mod db_types_tab;

pub trait DBTab {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> io::Result<()>;
    fn handle_input(&mut self, key: KeyEvent) -> io::Result<()>;
    fn is_disabled(&self) -> bool;
    fn set_disabled(&mut self, disabled: bool);
    fn get_title(&self) -> String;
}
