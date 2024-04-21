use anyhow::Result;
use ratatui::style::{Color, Modifier, Style};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write, path::PathBuf, rc::Rc};
use struct_patch::Patch;

pub type SharedTheme = Rc<Theme>;

#[derive(Debug)]
pub struct Theme {
    selected_tab: Color,
    command_fg: Color,
    selection_bg: Color,
    selection_fg: Color,
    cmdbar_bg: Color,
    cmdbar_extra_lines_bg: Color,
    disabled_fg: Color,
    enabled_fg: Color,
    danger_fg: Color,
    line_break: String,
    block_title_focused: Color,
}

impl Theme {
    pub fn scroll_bar_pos(&self) -> Style {
        Style::default().fg(self.selection_bg)
    }

    pub fn block(&self, focus: bool) -> Style {
        if focus {
            Style::default()
        } else {
            Style::default().fg(self.disabled_fg)
        }
    }

    pub fn title(&self, focused: bool) -> Style {
        if focused {
            Style::default()
                .fg(self.block_title_focused)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(self.disabled_fg)
        }
    }

    pub fn tab(&self, enabled: bool, selected: bool) -> Style {
        if selected {
            self.text(enabled, selected)
                .fg(self.selected_tab)
                .bg(Color::Reset)
                .add_modifier(Modifier::BOLD)
        } else {
            self.text(enabled, selected)
        }
    }

    pub fn text(&self, enabled: bool, selected: bool) -> Style {
        match (enabled, selected) {
            (false, false) => Style::default().fg(self.disabled_fg),
            (false, true) => Style::default().bg(self.disabled_fg),
            (true, false) => Style::default().fg(self.enabled_fg),
            (true, true) => Style::default().fg(self.command_fg),
        }
    }

    const fn apply_select(&self, style: Style, selected: bool) -> Style {
        if selected {
            style.bg(self.selection_bg).fg(self.selection_fg)
        } else {
            style
        }
    }

    pub fn text_danger(&self) -> Style {
        Style::default().fg(self.danger_fg)
    }

    pub fn line_break(&self) -> String {
        self.line_break.clone()
    }

    pub fn commandbar(&self, enabled: bool, line: usize) -> Style {
        if enabled {
            Style::default().fg(self.command_fg)
        } else {
            Style::default().fg(self.disabled_fg)
        }
        .bg(if line == 0 {
            self.cmdbar_bg
        } else {
            self.cmdbar_extra_lines_bg
        })
    }

    pub fn attention_block() -> Style {
        Style::default().fg(Color::Yellow)
    }

    pub fn init(theme_path: &PathBuf) -> Self {
        let mut theme = Self::default();
        theme
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            selected_tab: Color::Yellow,
            selection_bg: Color::Blue,
            selection_fg: Color::White,
            command_fg: Color::White,
            cmdbar_bg: Color::Blue,
            cmdbar_extra_lines_bg: Color::Blue,
            disabled_fg: Color::DarkGray,
            enabled_fg: Color::Blue,
            danger_fg: Color::Red,
            line_break: "¶".to_string(),
            block_title_focused: Color::Reset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    #[test]
    fn test_smoke() {
        let mut file = NamedTempFile::new().unwrap();

        writeln!(
            file,
            r"
(
	selection_bg: Some(White),
)
"
        )
        .unwrap();

        let theme = Theme::init(&file.path().to_path_buf());

        assert_eq!(theme.selection_fg, Theme::default().selection_fg);
        assert_eq!(theme.selection_bg, Color::White);
        assert_ne!(theme.selection_bg, Theme::default().selection_bg);
    }
}
