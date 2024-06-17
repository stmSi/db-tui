use std::fmt::{self, Debug, Formatter};

use quit_confirm_popup::QuitConfirmPopup;

pub mod connection_popup;
pub mod quit_confirm_popup;
pub mod style;

pub enum Popup<'a> {
    Connection(connection_popup::DbConnectionPopup<'a>),
    Quit(QuitConfirmPopup),
}

impl<'a> Debug for Popup<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Popup::Connection(connection_popup) => {
                write!(
                    f,
                    "New Popup Connection {:?}",
                    connection_popup.get_db_driver_name()
                )
            }
            Popup::Quit(quit_popup) => write!(f, "New Popup Quit {:?}", quit_popup),
        }
    }
}
