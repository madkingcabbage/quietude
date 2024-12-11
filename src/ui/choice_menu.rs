use log::debug;

use crate::types::Direction1D;

pub struct ChoiceMenu {
    pub index: usize,
    pub options: Vec<String>,
}

impl ChoiceMenu {
    pub fn new(options: Vec<String>) -> Self {
        ChoiceMenu {
            index: 0,
            options,
        }
    }

    pub fn move_cursor(&mut self, direction: Direction1D) {
        debug!("moved cursor");
        match direction {
            Direction1D::Up => {
                if self.index > 0 {
                    self.index -= 1;
                }
            }
            Direction1D::Down => {
                if self.index < self.options.len() - 1 {
                    self.index += 1;
                }
            }
        }
    }

    pub fn get_cursor_pos(&self) -> usize {
        self.index
    }
}

impl Default for ChoiceMenu {
    fn default() -> Self {
        Self { index: 0, options: vec![] }
    }
}
