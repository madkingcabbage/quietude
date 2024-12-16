use crate::types::Direction1D;

pub struct ChoiceMenu {
    pub index: usize,
    pub options: Vec<String>,
    on_exit: Option<Box<dyn FnOnce(&str)>>,
}

impl ChoiceMenu {
    pub fn new(options: Vec<String>, on_exit: impl FnOnce(&str) + 'static) -> Self {
        ChoiceMenu {
            index: 0,
            options,
            on_exit: Some(Box::new(on_exit)),
        }
    }

    pub fn move_cursor(&mut self, direction: Direction1D) {
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

    pub fn exit(&mut self) {
        let s = &self.options[self.index];
        (self.on_exit.take().unwrap())(s);
    }
}

impl Default for ChoiceMenu {
    fn default() -> Self {
        Self { index: 0, options: vec![], on_exit: None }
    }
}
