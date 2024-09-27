use tui_textarea::TextArea;

use super::{
    constants::{MAX_NAME_LENGTH, MIN_NAME_LENGTH},
    widgets::default_block,
};

pub fn validate_textarea_input(textarea: &mut TextArea<'_>, title: String) -> bool {
    let text = textarea.lines()[0].trim();
    if text.len() < MIN_NAME_LENGTH {
        textarea.set_block(default_block().title(title).title("(too short)"));
        false
    } else if text.len() > MAX_NAME_LENGTH {
        textarea.set_block(default_block().title(title).title("(too long)"));
        false
    } else {
        textarea.set_block(default_block().title(title));
        true
    }
}
