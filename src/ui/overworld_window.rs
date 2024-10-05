use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, style::{Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph}, Frame};

use crate::{constants::{MAX_COORDS, MIN_COORDS}, types::{Coords3D, Direction3D}, world::{entity::Entity, world::World}};

use super::{control_scheme::{ControlSchemeType, UiKey}, cursor::Cursor, traits::Screen, ui_callback::UiCallbackPreset};

pub struct OverworldWindow {
    pub cursor: Cursor,
}

impl OverworldWindow {
    pub fn new() -> Self {
        OverworldWindow {
            cursor: Cursor::default(),
        }
    }

    pub fn entity_span(entity: &Entity) -> Span<'static> {
        Span::styled("T", Style::default())
    }

    pub fn void_span() -> Span<'static> {
        Span::styled(".", Style::default())
    }

}

impl Screen for OverworldWindow {
    fn update(&mut self, world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
        let mut spans = vec![];
        let mut used_coords = vec![];
        for (_, entity) in world.active_chunk.get_entities() {
            spans.push((entity.coords, OverworldWindow::entity_span(entity)));
            used_coords.push(entity.coords);
        }

        for y in MIN_COORDS.1..=MAX_COORDS.1 {
            for x in MIN_COORDS.0..=MAX_COORDS.0 {
                let mut has_object = false;
                for z in MIN_COORDS.2..=MAX_COORDS.2 {
                    if used_coords.contains(&Coords3D(x, y, z)) {
                        has_object = true;
                    }
                }
                if !has_object {
                    spans.push((Coords3D(x, y, MIN_COORDS.2), OverworldWindow::void_span()));
                }
            }
        }

        spans.sort_by(|(coords1, _), (coords2, _)| coords1.partial_cmp(coords2).unwrap());

        if self.cursor.is_visible {
            let mut index = 0;
            for (i, (coords, span)) in spans.iter().enumerate() {
                if &self.cursor.coords == coords {
                    index = i;
                }
            }

            spans[index].1 = spans[index].1.clone().add_modifier(Modifier::REVERSED);
        }

        let mut lines = vec![];
        let mut line = vec![];
        let mut current_y = MIN_COORDS.1;
        for (coords, span) in &spans {
            if coords.1 != current_y {
                current_y = coords.1;
                lines.push(Line::from(line.clone()));
                line = vec![];
            }
            line.push(span.clone());
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Overworld")
            .padding(Padding::top(((area.height - (MAX_COORDS.1 - MIN_COORDS.1) as u16) / 2) - 1));
        let p = Paragraph::new(lines).block(block).centered();

        frame.render_widget(p, area);

        Ok(())
    }

    fn get_refresh_rate(&self) -> u16 {
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
                UiKey::MoveNorth => return Some(UiCallbackPreset::MovePlayer(Direction3D::North)),
                UiKey::MoveEast => return Some(UiCallbackPreset::MovePlayer(Direction3D::East)),
                UiKey::MoveWest => return Some(UiCallbackPreset::MovePlayer(Direction3D::West)),
                UiKey::MoveSouth => return Some(UiCallbackPreset::MovePlayer(Direction3D::South)),
                UiKey::Quit => todo!(),
                _ => {}
            }
        }

        None
    }
}
