use std::default;

use ratatui::style::Style;
use serde::{Deserialize, Serialize};

use crate::types::{Color, Coords3D, FormattedString, FormattedText};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Log {
    pub contents: Vec<FormattedString<LogStyle>>,
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
            contents: vec![],
        }
    }

    pub fn print_formatted_string(&mut self, string: FormattedString<LogStyle>) {
        self.contents.push(string);
    }

}


impl From<LogStyle> for Style {
    fn from(value: LogStyle) -> Self {
        Style::new()
    }
}
