use std::borrow::Cow;

use unicode_truncate::UnicodeTruncateStr;
use unicode_width::UnicodeWidthStr;

use crate::keys::SharedKeyConfig;

pub mod order {
    pub const RARE_ACTION: i8 = 30;
    pub const NAV: i8 = 20;
    pub const AVERAGE: i8 = 10;
    pub const PRIORITY: i8 = 1;
}

pub static DB_TYPES: &str = "DB Types";
pub static DB_CONNECTIONS: &str = "DB Connections";
pub static DB_DATABASES: &str = "Database";
pub static DB_TABLES_AND_SCHEMAS: &str = "Tables & Schemas";
pub static POPUP_SUCCESS_COPY: &str = "Copied Text";

pub mod symbol {
    pub const CHECKMARK: &str = "\u{2713}"; //✓
    pub const SPACE: &str = "\u{02FD}"; //˽
    pub const EMPTY_SPACE: &str = " ";
    pub const FOLDER_ICON_COLLAPSED: &str = "\u{25b8}"; //▸
    pub const FOLDER_ICON_EXPANDED: &str = "\u{25be}"; //▾
    pub const EMPTY_STR: &str = "";
    pub const ELLIPSIS: char = '\u{2026}'; // …
}

pub fn tab_db_types(_key_config: &SharedKeyConfig) -> String {
    DB_TYPES.to_string()
}

pub fn tab_db_connections(_key_config: &SharedKeyConfig) -> String {
    DB_CONNECTIONS.to_string()
}

pub fn tab_db_databases(_key_config: &SharedKeyConfig) -> String {
    DB_DATABASES.to_string()
}

pub fn tab_db_tables_and_schema(_key_config: &SharedKeyConfig) -> String {
    DB_TABLES_AND_SCHEMAS.to_string()
}

pub fn tab_divider(_key_config: &SharedKeyConfig) -> String {
    " | ".to_string()
}
pub fn cmd_splitter(_key_config: &SharedKeyConfig) -> String {
    " ".to_string()
}

pub fn msg_title_error(_key_config: &SharedKeyConfig) -> String {
    "Error".to_string()
}
pub fn msg_title_info(_key_config: &SharedKeyConfig) -> String {
    "Info".to_string()
}

pub fn copy_success(s: &str) -> String {
    format!("{POPUP_SUCCESS_COPY} \"{s}\"")
}

pub fn ellipsis_trim_start(s: &str, width: usize) -> Cow<str> {
    if s.width() <= width {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(format!(
            "[{}]{}",
            symbol::ELLIPSIS,
            s.unicode_truncate_start(width.saturating_sub(3 /* front indicator */))
                .0
        ))
    }
}
