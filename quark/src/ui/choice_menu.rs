use anyhow::Result;
use crossterm::event::KeyEvent;
use quietude::{
    types::Direction1D,
    ui::choice_menu::ChoiceMenu,
    world::{log::LogStyle, world::World},
};
use ratatui::{
    prelude::Rect, style::{Modifier, Stylize}, text::Line, widgets::{Block, Clear, Paragraph}, Frame
};

use super::{
    control_scheme::{ControlSchemeType, UiKey},
    traits::Screen,
    ui_callback::UiCallbackPreset,
};

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
                    return Some(UiCallbackPreset::MoveChoiceMenuCursor(Direction1D::Down))
                }
                UiKey::MoveUp => {
                    return Some(UiCallbackPreset::MoveChoiceMenuCursor(Direction1D::Up))
                }
                UiKey::Confirm => {
                    return Some(UiCallbackPreset::ExitChoiceMenu(
                        self.options[self.index].clone(),
                    ))
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
