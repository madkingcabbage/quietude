use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, style::Style, text::{Line, Span}, widgets::{Block, Borders, Paragraph}, Frame};

use crate::{types::Direction3D, world::{entity::Entity, world::World}};

use super::{control_scheme::{ControlSchemeType, UiKey}, traits::Screen, ui_callback::UiCallbackPreset};

pub struct OverworldWindow;

impl OverworldWindow {
    pub fn new() -> Self {
        OverworldWindow {}
    }

    pub fn entity_span(entity: &Entity) -> Span<'static> {
        Span::styled(".", Style::default())
    }
}

impl Screen for OverworldWindow {
    fn update(&mut self, world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
        let mut spans = vec![];
        for (_, entity) in world.active_chunk.get_entities() {
            spans.push((entity.coords, OverworldWindow::entity_span(entity)));
        }

        spans.sort_by(|(coords1, _), (coords2, _)| coords1.partial_cmp(coords2).unwrap());

        let spans: Vec<_> = spans
            .iter()
            .map(|(_, entity)| entity.clone())
            .collect();

        let line = Line::from(spans);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Overworld");
        let p = Paragraph::new(line).block(block);

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
