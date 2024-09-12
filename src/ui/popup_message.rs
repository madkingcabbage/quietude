use crossterm::event::KeyEvent;
use tui_textarea::TextArea;

use super::{
    control_scheme::{ControlSchemeType, UiKey},
    ui_callback::UiCallbackPreset, utils::validate_textarea_input,
};

pub enum PopupMessage {
    Ok(String),
    Err(String),
    SaveNameDialog,
}

impl PopupMessage {
    pub fn consumes_input(
        &self,
        popup_input: &mut TextArea<'static>,
        key: KeyEvent,
        scheme: &ControlSchemeType,
    ) -> Option<UiCallbackPreset> {
        match self {
            PopupMessage::SaveNameDialog => {
                let name = popup_input.lines()[0].clone();
                if validate_textarea_input(popup_input, "Save name".into()) {
                    return Some(UiCallbackPreset::SetSavename(name));
                }
            }
            _ => {
                if scheme.code_yields_key(key.code, UiKey::NoToDialog)
                    || scheme.code_yields_key(key.code, UiKey::YesToDialog)
                {
                    return Some(UiCallbackPreset::CloseUiPopup);
                }
            }
        }
        None
    }
}
