use std::{io, sync::mpsc};

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::AppEvent;

pub mod db_connections_tab;
pub mod db_databases_tab;
pub mod db_tables_tab;
pub mod db_types_tab;

pub trait DBTab {
    fn draw(&mut self, frame: &mut Frame, area: Rect) -> io::Result<()>;
    fn handle_input(
        &mut self,
        key: KeyEvent,
        app_event_bus: &mpsc::Sender<AppEvent>,
    ) -> io::Result<()>;
    fn is_disabled(&self) -> bool;
    fn set_disabled(&mut self, disabled: bool);
    fn get_title(&self) -> String;
}
