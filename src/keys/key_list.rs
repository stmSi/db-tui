use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
use struct_patch::traits::Patch as PatchTrait;
use struct_patch::Patch;

#[derive(Debug, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub struct GituiKeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl GituiKeyEvent {
    pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }
}

pub fn key_match(ev: &KeyEvent, binding: GituiKeyEvent) -> bool {
    ev.code == binding.code && ev.modifiers == binding.modifiers
}

impl PartialEq for GituiKeyEvent {
    fn eq(&self, other: &Self) -> bool {
        let ev: KeyEvent = self.into();
        let other: KeyEvent = other.into();
        ev == other
    }
}

impl From<&GituiKeyEvent> for KeyEvent {
    fn from(other: &GituiKeyEvent) -> Self {
        Self::new(other.code, other.modifiers)
    }
}

#[derive(Debug, Clone, Patch)]
#[patch_derive(Deserialize, Debug)]
pub struct KeysList {
    pub exit: GituiKeyEvent,
    pub quit: GituiKeyEvent,
    pub move_left: GituiKeyEvent,
    pub move_left_h: GituiKeyEvent,
    pub move_right: GituiKeyEvent,
    pub move_right_l: GituiKeyEvent,
    pub move_up: GituiKeyEvent,
    pub move_up_k: GituiKeyEvent,
    pub move_down: GituiKeyEvent,
    pub move_down_j: GituiKeyEvent,
}

#[rustfmt::skip]
impl Default for KeysList {
	fn default() -> Self {
		Self {
			exit: GituiKeyEvent::new(KeyCode::Char('c'),  KeyModifiers::CONTROL),
			quit: GituiKeyEvent::new(KeyCode::Char('q'),  KeyModifiers::empty()),
			move_left: GituiKeyEvent::new(KeyCode::Left,  KeyModifiers::empty()),
			move_left_h: GituiKeyEvent::new(KeyCode::Char('h'),  KeyModifiers::empty()),
			move_right: GituiKeyEvent::new(KeyCode::Right,  KeyModifiers::empty()),
			move_right_l: GituiKeyEvent::new(KeyCode::Char('l'),  KeyModifiers::empty()),
			move_up: GituiKeyEvent::new(KeyCode::Up,  KeyModifiers::empty()),
			move_up_k: GituiKeyEvent::new(KeyCode::Char('k'),  KeyModifiers::empty()),
			move_down: GituiKeyEvent::new(KeyCode::Down,  KeyModifiers::empty()),
			move_down_j: GituiKeyEvent::new(KeyCode::Char('j'),  KeyModifiers::empty()),
		}
	}
}

impl KeysList {
    pub fn init(file: PathBuf) -> Self {
        let mut keys_list = Self::default();
        if let Ok(f) = File::open(file) {
            match ron::de::from_reader(f) {
                Ok(patch) => keys_list.apply(patch),
                Err(e) => {
                    log::error!("KeysList parse error: {e}");
                }
            }
        }
        keys_list
    }
}
