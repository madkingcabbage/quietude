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
    Attribute,
    Value,
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
        match value {
            LogStyle::Default => Style::new(),
            LogStyle::Emphasis(color) => todo!(),
            LogStyle::Attribute =>Style::default().fg(ratatui::style::Color::Yellow).bg(ratatui::style::Color::DarkGray),
            LogStyle::Value => Style::default().fg(ratatui::style::Color::Cyan),
        }
    }
}
