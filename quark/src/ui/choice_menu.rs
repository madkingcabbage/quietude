use anyhow::Result;
use crossterm::event::KeyEvent;
use quietude::{
    app::App, types::Direction1D, world::{log::LogStyle, world::World}
};
use ratatui::{
    prelude::Rect, style::{Modifier, Stylize}, text::Line, widgets::{Block, Clear, Paragraph}, Frame
};

use super::{
    control_scheme::{ControlSchemeType, UiKey}, traits::Screen, ui::Ui, ui_callback::UiCallbackPreset
};

pub struct ChoiceMenu {
    pub index: usize,
    pub options: Vec<String>,
    pub on_exit: Option<fn(&str, &mut Ui) -> Result<()>>,
}

impl ChoiceMenu {
    pub fn new(options: Vec<String>, on_exit: fn(&str, &mut Ui) -> Result<()>) -> Self {
        ChoiceMenu {
            index: 0,
            options,
            on_exit: Some(on_exit),
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

    pub fn exit(&self, ui: &mut Ui) -> Result<()> {
        let s = &self.options[self.index];
        (self.on_exit.as_ref().unwrap().clone())(s, ui)
    }
}

impl Default for ChoiceMenu {
    fn default() -> Self{
        Self { index: 0, options: vec![], on_exit: None }
    }
}

impl Screen for ChoiceMenu {
    fn update(&mut self, _world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, _world: &World, area: Rect) -> Result<()> {
        let mut lines = vec![];

        for (i, option) in self.options.iter().enumerate() {
            let mut line = Line::styled(option, LogStyle::Value);
            if i == self.index {
                line = line.add_modifier(Modifier::REVERSED);
            }
            lines.push(line);
        }

        let p = Paragraph::new(lines).block(Block::bordered());
        frame.render_widget(Clear, area);
        frame.render_widget(p, area);

        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        scheme: ControlSchemeType,
        _world: &World,
    ) -> Option<UiCallbackPreset> {
        let keys = match scheme.keys_from_code(key_event.code) {
            Some(keys) => keys,
            None => &vec![],
        };

        for key in keys {
            match key {
                UiKey::MoveDown => {
                    return Some(UiCallbackPreset::MoveChoiceMenuCursor(Direction1D::Down));
                }
                UiKey::MoveUp => {
                    return Some(UiCallbackPreset::MoveChoiceMenuCursor(Direction1D::Up));
                }
                UiKey::Confirm => {
                    return Some(UiCallbackPreset::ChoiceMenuSelectAndExit(
                        self.options[self.index].clone(),
                    ));
                }
                UiKey::ExitSubmenu => {
                    return Some(UiCallbackPreset::ExitChoiceMenu);
                }
                _ => {}
            }
        }

        return None;
    }

    fn refresh_rate(&self) -> u16 {
        60
    }
}
