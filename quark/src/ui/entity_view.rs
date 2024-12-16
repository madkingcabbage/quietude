use anyhow::{anyhow, Result};
use crossterm::event::KeyEvent;
use quietude::{
    types::{Coords3D, Direction1D, FormattedString, FormattedText}, ui::{traits::ChoiceAttribute}, world::{
        chunk::Chunk, entity::{Entity, EntityAttribute, EntityAttributeChoice, EntityAttributeText, EntityType, Opacity, Size}, log::LogStyle, world::World
    }
};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};
use tui_textarea::TextArea;

use super::{
    choice_menu::ChoiceMenu, control_scheme::{ControlSchemeType, UiKey}, traits::Screen, ui_callback::UiCallbackPreset
};

pub struct EntityView {
    entity: Option<Entity>,
    pub state: EntityViewState,
    cursor_pos: usize,
    text_attr_editor: TextAttributeEditor,
}

#[derive(Default)]
pub enum EntityViewState {
    #[default]
    Main,
    FormattedStringInput,
}

pub struct TextAttributeEditor {
    pub attr: EntityAttributeText,
    pub text_area: TextArea<'static>,
}

#[derive(Default)]
pub struct ChoiceAttributeEditor {
    pub attr: EntityAttributeChoice,
    pub menu: ChoiceMenu,
}

impl EntityView {
    pub fn new() -> Self {
        EntityView {
            entity: None,
            state: EntityViewState::default(),
            cursor_pos: 0,
            text_attr_editor: TextAttributeEditor {
                attr: EntityAttributeText::default(),
                text_area: TextArea::default(),
            },
        }
    }

    pub fn finish(&mut self) -> Result<Entity> {
        let entity = self
            .entity
            .clone()
            .ok_or(anyhow!("tried to get ownership of empty entity"));
        self.entity = None;

        entity
    }

    pub fn start_editor(&mut self, coords: Coords3D, chunk: &Chunk) {
        self.cursor_pos = 0;
        self.entity = Some(
            chunk
                .get_entity_from_coords(&coords)
                .unwrap_or(&Entity::new(&coords))
                .clone(),
        );
    }

    pub fn start_attr_text_editor(
        &mut self,
        attr: EntityAttributeText,
        default: &FormattedString<LogStyle>,
    ) {

        self.text_attr_editor = TextAttributeEditor {
            attr,
            text_area: TextArea::from(vec![format!("{default}")]),
        };
        self.state = EntityViewState::FormattedStringInput;
    }

    pub fn set_text_attribute(&mut self, attr: EntityAttributeText, value: &str) {
        match attr {
            EntityAttributeText::Name => {
                self.entity.as_mut().unwrap().name = FormattedString::raw(&None, value)
            }
            EntityAttributeText::Description => {
                self.entity.as_mut().unwrap().description = FormattedString::raw(&None, value)
            }
        }
    }

    pub fn set_choice_attr(&mut self, attr: EntityAttributeChoice, s: &str) -> Result<()> {
        let entity = self.entity.as_mut().ok_or(anyhow!("tried to access empty entity"))?;
        match attr {
            EntityAttributeChoice::Type => entity.entity_type = EntityType::from_str(s)?.clone(),
            EntityAttributeChoice::IsRooted => entity.is_rooted = Some(bool::from_str(s)?.clone()),
            EntityAttributeChoice::HasAgency => entity.has_agency = Some(bool::from_str(s)?.clone()),
            EntityAttributeChoice::Allegiance => todo!(),
            EntityAttributeChoice::Opacity => entity.opacity = Some(Opacity::from_str(s)?.clone()),
            EntityAttributeChoice::Size => entity.size = Some(Size::from_str(s)?.clone()),
        }

        Ok(())

    }

    pub fn get_current_attribute(&self) -> Result<EntityAttribute> {
        let (attr, _) = Self::index_to_attribute_lookup(self.cursor_pos, self.entity.as_ref().ok_or(anyhow!("tried to get current attribute of empty entity"))?)?;
        Ok(attr.clone())
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
    
    pub fn add_attribute(&mut self, attr: &EntityAttribute) -> Result<()> {
        self.entity.as_mut().ok_or(anyhow!("tried to access empty entity"))?.add_attribute(attr)
    }

    pub fn remove_attribute(&mut self, attr: &EntityAttribute) -> Result<()> {
        self.entity.as_mut().ok_or(anyhow!("tried to access empty entity"))?.remove_attribute(attr)
    }

    pub fn has_attribute(&self, attr: &EntityAttribute) -> Result<bool> {
        Ok(self.entity.as_ref().ok_or(anyhow!("tried to access empty entity"))?.has_attribute(attr))
    }

    pub fn attribute_list(
        entity: &Entity,
    ) -> Result<Vec<(FormattedText<LogStyle>, FormattedString<LogStyle>)>> {
        let mut index = 0;
        let mut list = vec![];
        for attr in EntityAttribute::attribute_order() {
            if entity.has_attribute(attr) {
                let (key, value) = EntityView::index_to_attribute_lookup(index, entity)?;
                list.push((
                    FormattedText::new(&format!("{key}"), LogStyle::Attribute),
                    value,
                ));
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
                if self.cursor_pos
                    < self
                        .entity
                        .as_ref()
                        .unwrap_or_else(|| {
                            panic!("tried to move cursor without actively editing entity")
                        })
                        .attribute_count() - 1
                {
                    self.cursor_pos += 1;
                }
            }

        }
    }

    pub fn validate_cursor_pos(&mut self) {
        let max = self.entity.as_ref().unwrap_or_else(|| panic!("tried to access empty entity")).attribute_count() - 1;
        if max < self.cursor_pos {
            self.cursor_pos = max;
        }
    }
}

impl Screen for EntityView {
    fn update(&mut self, _world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
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
            let spacer = FormattedText::new(": ", LogStyle::Attribute);
            s.insert(0, attr);
            s.insert(1, spacer);
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
            "{}", entity.coords
        ));

        let p = Paragraph::new(lines).block(block);
        frame.render_widget(Clear, area);
        frame.render_widget(p, area);

        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4),
                Constraint::Min(52),
                Constraint::Length(4),
            ])
            .split(area);
        let block_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(36),
                Constraint::Length(2),
            ])
            .split(layout[1]);
        let layout = Layout::default()

            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(5),
                Constraint::Min(50),
                Constraint::Length(5),
            ])
            .split(area);
        let text_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(34),
                Constraint::Length(3),
            ])
            .split(layout[1]);
        
        match self.state {
            EntityViewState::Main => {}
            EntityViewState::FormattedStringInput => {
                let b = Block::bordered().title(format!("{}", self.text_attr_editor.attr));
                frame.render_widget(Clear, block_layout[1]);
                frame.render_widget(b, block_layout[1]);
                frame.render_widget(&self.text_attr_editor.text_area, text_layout[1]);
            }
        }

        Ok(())
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        scheme: ControlSchemeType,
        world: &World,
    ) -> Option<UiCallbackPreset> {
        let keys = match scheme.keys_from_code(key_event.code) {
            Some(keys) => keys,
            None => &vec![],
        };

        match self.state {
            EntityViewState::Main => {}
            EntityViewState::FormattedStringInput => {
                for key in keys {
                    if *key == UiKey::ExitSubmenu {
                        let lines = self
                            .text_attr_editor
                            .text_area
                            .lines()
                            .iter()
                            .map(|line| line.clone())
                            .collect();
                        return Some(UiCallbackPreset::ExitStringEditor(
                            self.text_attr_editor.attr.clone(),
                            lines,
                        ));
                    }
                }

                self.text_attr_editor.text_area.input(key_event);

                return None;
            }
        }

        for key in keys {
            match key {
                UiKey::MoveNorth => {
                    return Some(UiCallbackPreset::MoveEntityViewCursor(Direction1D::Up))
                }
                UiKey::MoveSouth => {
                    return Some(UiCallbackPreset::MoveEntityViewCursor(Direction1D::Down))
                }
                UiKey::Confirm => {
                    let (key, value) = EntityView::index_to_attribute_lookup(
                        self.cursor_pos,
                        &self.entity.as_ref().unwrap(),
                    )
                    .unwrap_or_else(|e| {
                        panic!("{} while matching cursor index to attribute", e.to_string())
                    });
                    return Some(UiCallbackPreset::EditEntityAttribute(key.clone(), value));
                }
                UiKey::AddItem => {
                    return Some(UiCallbackPreset::AddEntityAttribute);
                }
                UiKey::RemoveItem => {
                    let attr = self.get_current_attribute().unwrap_or_else(|e| {
                        panic!("{} while removing entity attribute", e.to_string())
                    });
                    return Some(UiCallbackPreset::RemoveEntityAttribute(attr));
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
        60
    }
}

