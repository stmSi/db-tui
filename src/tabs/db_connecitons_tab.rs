use std::io::{self, Result};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{block::*, *},
};

use super::DBTab;

#[derive(Debug)]
pub struct DbConnectionsTab {
    pub connections: Vec<String>,
    pub selected: usize,
    pub list_state: ListState,
    pub num_tabs: usize,
}
