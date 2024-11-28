use quietude::{types::FormattedString, world::log::LogStyle};
use serde::{Deserialize, Serialize};

use crate::ui::popup_message::PopupStyle;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum Message {
    Popup(FormattedString<PopupStyle>),
    Log(FormattedString<LogStyle>),
}
