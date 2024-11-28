use anyhow::{anyhow, Result};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::Rect,
    style::{Modifier, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{types::{Direction1D, FormattedString}, world::world::World};

use super::{
    control_scheme::{ControlSchemeType, UiKey},
    traits::Screen,
    ui_callback::UiCallbackPreset,
};

pub struct DialogueWindow {
    active_choice: usize,
}

impl DialogueWindow {
    pub fn new() -> Self {
        DialogueWindow { active_choice: 0 }
    }

    pub fn move_highlight(&mut self, direction: Direction1D, max_choice: usize) -> Result<()> {
        let old_index = self.active_choice;
        let new_index = match direction {
            Direction1D::Up => old_index - 1,
            Direction1D::Down => old_index + 1,
        };

        if (new_index <= max_choice) && (new_index >= 0) {
            self.active_choice = new_index;
        }

        Ok(())
    }
}

impl Screen for DialogueWindow {
    fn update(&mut self, world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
        if world.dialogue_tree.as_ref().is_some() {
            let active_node = world
                .dialogue_tree
                .as_ref()
                .unwrap_or(Err(anyhow!("no dialogue tree currently active"))?)
                .get_active_node()?;
            let speaker_dialogue = active_node.speaker_dialogue();
            let choices = active_node.choices_text(world)?;

            let speaker_spans = FormattedString::into_spans(speaker_dialogue);

            let mut choice_lines = vec![];
            for (i, choice) in choices.iter().enumerate() {
                if i == self.active_choice {
                    let choice_spans = FormattedString::into_spans(choice);
                    choice_lines.push(Line::from(choice_spans).add_modifier(Modifier::REVERSED));
                }
            }

            let mut lines = vec![Line::from(speaker_spans)];
            lines.append(&mut choice_lines);

            let block = Block::default()
                .borders(Borders::ALL)
                .title(world.dialogue_tree.as_ref().unwrap().speaker_name.clone());

            let p = Paragraph::new(lines).block(block);
            frame.render_widget(p, area);
        } else {
            let p = Paragraph::default().block(Block::default().borders(Borders::ALL));
            frame.render_widget(p, area);
        }
        Ok(())
    }

    fn refresh_rate(&self) -> u16 {
        60
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        scheme: ControlSchemeType,
        world: &World,
    ) -> Option<UiCallbackPreset> {
        let keys = match scheme.keys_from_code(key_event.code) {
            Some(keys) => keys,
            None => return None,
        };

        for key in keys {
            match key {
                UiKey::MoveNorth => {
                    return Some(UiCallbackPreset::MoveDialogueHighlight(Direction1D::Up))
                }
                UiKey::MoveSouth => {
                    return Some(UiCallbackPreset::MoveDialogueHighlight(Direction1D::Down))
                }
                UiKey::Confirm => {
                    return Some(UiCallbackPreset::SelectDialogueEntry(self.active_choice))
                }
                _ => {}
            }
        }

        None
    }
}
