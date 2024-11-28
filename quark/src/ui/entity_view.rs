use anyhow::{anyhow, Result};
use crossterm::event::KeyEvent;
use quietude::{
    types::{Coords3D, Direction1D, FormattedString, FormattedText},
    world::{
        chunk::Chunk,
        entity::{Entity, EntityAttribute},
        log::LogStyle,
        world::World,
    },
};
use ratatui::{
    layout::Rect,
    style::{Modifier, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{
    control_scheme::{ControlSchemeType, UiKey},
    traits::Screen,
    ui_callback::UiCallbackPreset,
};

pub struct EntityView {
    entity: Option<Entity>,
    state: EntityViewState,
    cursor_pos: usize,
}

#[derive(Default)]
pub enum EntityViewState {
    #[default]
    Main,
    StringChoice,
    FormattedStringInput,
}

impl EntityView {
    pub fn new() -> Self {
        EntityView {
            entity: None,
            state: EntityViewState::default(),
            cursor_pos: 0,
        }
    }

    pub fn finish(&mut self) -> Result<Entity> {
        let entity = self.entity.clone().ok_or(anyhow!("tried to get ownership of empty entity"));
        self.entity = None;

        entity
    }

    pub fn edit_entity(&mut self, coords: Coords3D, chunk: &Chunk) {
        self.cursor_pos = 0;
        self.entity = Some(
            chunk
                .get_entity_from_coords(&coords)
                .unwrap_or(&Entity::new(&coords))
                .clone(),
        );
    }

    pub fn index_to_attribute_lookup(
        index: usize,
        entity: &Entity,
    ) -> Result<(&EntityAttribute, FormattedString<LogStyle>)> {
        let mut attr_count = 0;
        for attr in EntityAttribute::attribute_order() {
            if entity.has_attribute(attr) {
                let value = entity.get_attribute_value(attr);
                if index == attr_count {
                    return Ok((attr, value));
                }
                attr_count += 1;
            }
        }
        Err(anyhow!(format!(
            "index {index} does not compute to a valid attribute"
        )))
    }

    pub fn attribute_list(
        entity: &Entity,
    ) -> Result<Vec<(FormattedText<LogStyle>, FormattedString<LogStyle>)>> {
        let mut index = 0;
        let mut list = vec![];
        for attr in EntityAttribute::attribute_order() {
            if entity.has_attribute(attr) {
                let (key, value) = EntityView::index_to_attribute_lookup(index, entity)?;
                list.push((FormattedText::new(&format!("{key}"), LogStyle::Value), value));
                index += 1;
            }
        }
        Ok(list)
    }

    pub fn move_cursor(&mut self, direction: &Direction1D) {
        match direction {
            Direction1D::Up => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            Direction1D::Down => {
                if self.cursor_pos < self.entity.as_ref().unwrap_or_else(|| panic!("tried to move cursor without actively editing entity")).attribute_count() {
                    self.cursor_pos += 1;
                }
            }
        }
    }
}

impl Screen for EntityView {
    fn update(&mut self, _world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, _world: &World, area: Rect) -> Result<()> {
        self.entity
            .as_ref()
            .ok_or(anyhow!("tried to view an empty entity"))?;
        let entity = self.entity.as_ref().unwrap();

        let mut lines = vec![];
        let mut strings = vec![];

        let list = EntityView::attribute_list(entity)?;
        for (attr, value) in list {
            let attr = attr.truncate(16);
            let mut s = value.truncate(32);
            let spacer = FormattedText::new(": ", LogStyle::default());
            s.insert(0, attr);
            s.insert(0, spacer);
            strings.push(s.clone());
        }

        for (i, s) in strings.iter().enumerate() {
            let spans = FormattedString::into_spans(&s);
            let line = if self.cursor_pos == i {
                Line::from(spans).add_modifier(Modifier::REVERSED)
            } else {
                Line::from(spans)
            };
            lines.push(line);
        }

        let block = Block::default().borders(Borders::ALL).title(format!(
            "{}, {}, {}",
            entity.coords.0, entity.coords.1, entity.coords.2
        ));

        let p = Paragraph::new(lines).block(block);
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
            None => return None,
        };

        for key in keys {
            match key {
                UiKey::MoveNorth => {
                    return Some(UiCallbackPreset::MoveEntityViewCursor(Direction1D::Up))
                }
                UiKey::MoveSouth => {
                    return Some(UiCallbackPreset::MoveEntityViewCursor(Direction1D::Down))
                }
                UiKey::Confirm => {
                    let (key, value) = EntityView::index_to_attribute_lookup(self.cursor_pos, &self.entity.as_ref().unwrap()).unwrap_or_else(|e| panic!("{} while matching cursor index to attribute", e.to_string()));
                    return Some(UiCallbackPreset::EditEntityAttribute(key.clone(), value));
                }
                UiKey::ExitSubmenu => {
                    return Some(UiCallbackPreset::ExitEntityView);
                }
                _ => {}
            }
        }

        None
    }

    fn refresh_rate(&self) -> u16 {
        todo!()
    }
}
