use std::default;

use ratatui::style::Style;
use serde::{Deserialize, Serialize};

use crate::types::Color;

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Log {
    pub contents: Vec<(String, LogStyle)>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub enum LogStyle {
    #[default]
    Default,
    Emphasis(Color),
}

impl Log {
    pub fn new() -> Self {
        Log {
            contents: Vec::new(),
        }
    }

    pub fn print_to(&mut self, s: &str, style: LogStyle) {
        self.contents.push((String::from(s), style));
    }
}

impl From<LogStyle> for Style {
    fn from(value: LogStyle) -> Self {
        Style::new()
    }
}
