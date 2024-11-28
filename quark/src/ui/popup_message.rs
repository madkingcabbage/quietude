use crossterm::event::KeyEvent;
use quietude::types::{Color, FormattedString};
use ratatui::style::Style;
use serde::{Deserialize, Serialize};
use tui_textarea::TextArea;

use super::{
    control_scheme::{ControlSchemeType, UiKey},
    ui_callback::UiCallbackPreset,
};

pub enum PopupMessage {
    Ok(FormattedString<PopupStyle>),
    Err(FormattedString<PopupStyle>),
}

#[derive(Clone, Default, Deserialize, Serialize, Debug, PartialEq)]
pub enum PopupStyle {
    #[default]
    Default,
    Emphasis(Color),
    Error,
}

impl PopupMessage {
    pub fn consumes_input(
        &self,
        popup_input: &mut TextArea<'static>,
        key: KeyEvent,
        scheme: &ControlSchemeType,
    ) -> Option<UiCallbackPreset> {
        if scheme.code_yields_key(key.code, UiKey::NoToDialog)
            || scheme.code_yields_key(key.code, UiKey::YesToDialog)
        {
            return Some(UiCallbackPreset::CloseUiPopup);
        }
        None
    }
}

impl From<PopupStyle> for Style {
    fn from(value: PopupStyle) -> Self {
        Style::new()
    }
}
