use anyhow::{anyhow, Result};
use crossterm::event::KeyEvent;
use quietude::{types::{Direction1D, FormattedString}, world::{dialogue::DialogueTree, world::World}};
use ratatui::{prelude::Rect, style::{Modifier, Stylize}, text::Line, widgets::{Block, Borders, Paragraph}, Frame};

use super::{control_scheme::{ControlSchemeType, UiKey}, traits::Screen, ui_callback::UiCallbackPreset};

pub struct DialogueEditor {
    tree: Option<DialogueTree>,
    pub state: DialogueEditorState,
    cursor_pos: usize,
}

pub enum DialogueEditorState {
    Main,
    EditingName,
    EditingOption,
}

impl DialogueEditor {
    pub fn new() -> Self {
        DialogueEditor {
            tree: None,
            state: DialogueEditorState::Main,
            cursor_pos: 0,
        }
    }

    pub fn edit_dialogue(&mut self, tree: &DialogueTree) {
        self.cursor_pos = 0;
        self.tree = Some(tree.clone());
    }

    pub fn finish(&mut self) -> Result<DialogueTree> {
        let tree = self.tree.clone().ok_or(anyhow!("tried to get ownership of empty dialogue tree"));
        self.tree = None;

        tree
    }

    pub fn move_highlight(&mut self, direction: &Direction1D) -> Result<()> {
        let node = self.tree.as_ref().unwrap_or_else(|| panic!("tried to move cursor without actively editing dialogue tree")).get_active_node()?;

        match direction {
            Direction1D::Up => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            Direction1D::Down => {
                if self.cursor_pos < node.choices_count_unconditional() {
                    self.cursor_pos += 1;
                }
            },
        }

        Ok(())
    }
}

impl Screen for DialogueEditor {
    fn update(&mut self, _world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, _world: &World, area: Rect) -> Result<()> {
        let node = self.tree.as_ref().unwrap_or_else(|| panic!("tried to render an inactive dialogue tree")).get_active_node()?;

        let speaker_dialogue = node.speaker_dialogue();
        let choices = node.choices_text_unconditional()?;

        let mut lines = vec![
            Line::from(FormattedString::into_spans(speaker_dialogue)),
            Line::from(""),
            Line::from("-------------"),
            Line::from(""),
        ];

        for (i, choice) in choices.iter().enumerate() {
            let spans = FormattedString::into_spans(choice);
            let line = if self.cursor_pos == i {
                Line::from(spans).add_modifier(Modifier::REVERSED)
            } else {
                Line::from(spans)
            };
            lines.push(line);
        }

        let block = Block::default().borders(Borders::ALL).title(format!(
            "{}", self.tree.as_ref().unwrap().speaker_name
        ));

        let p = Paragraph::new(lines).block(block);
        frame.render_widget(p, area);
        
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent, scheme: ControlSchemeType, _world: &World)
        -> Option<UiCallbackPreset> {
        let keys = match scheme.keys_from_code(key_event.code) {
            Some(keys) => keys,
            None => return None,
        };

        for key in keys {
            match key {
                UiKey::MoveUp => return Some(UiCallbackPreset::MoveDialogueEditorCursor(Direction1D::Up)),
                UiKey::MoveDown => return Some(UiCallbackPreset::MoveDialogueEditorCursor(Direction1D::Down)),
                UiKey::ExitSubmenu => return Some(UiCallbackPreset::ExitDialogueEditor),
                _ => {}
            }
        }

        None
    }

    fn refresh_rate(&self) -> u16 {
        60
    }


}
