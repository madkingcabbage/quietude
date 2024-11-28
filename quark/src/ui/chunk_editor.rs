use anyhow::Result;
use crossterm::event::KeyEvent;
use quietude::{
    constants::{MAX_COORDS, MIN_COORDS}, types::{Coords3D, Direction3D}, world::{chunk::Chunk, entity::Entity, world::World}
};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style, Stylize}, text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph}, Frame};

use super::{control_scheme::{ControlSchemeType, UiKey}, cursor::Cursor, entity_view::EntityView, traits::Screen, ui_callback::UiCallbackPreset};

pub struct ChunkEditor {
    pub state: ChunkEditorState,
    pub cursor: Cursor,
    pub entity_view: EntityView,
}

#[derive(PartialEq, Eq, Default)]
pub enum ChunkEditorState {
    #[default]
    Main,
    EntityView,
}

impl ChunkEditor {
    pub fn new() -> Self {
        ChunkEditor {
            state: ChunkEditorState::default(),
            cursor: Cursor::default(),
            entity_view: EntityView::new(),
        }
    }

    pub fn edit_entity(&mut self, coords: Coords3D, chunk: &Chunk) -> Result<()> {
        self.entity_view.edit_entity(coords, chunk);
        self.state = ChunkEditorState::EntityView;

        Ok(())
    }

    pub fn entity_span(entity: &Entity) -> Span<'static> {
        Span::styled("T", Style::default())
    }

    pub fn void_span() -> Span<'static> {
        Span::styled(".", Style::default())
    }
}

impl Screen for ChunkEditor {
    fn update(&mut self, _world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
        let mut spans = vec![];
        let mut used_coords = vec![];
        for (_, entity) in world.active_chunk.get_entities() {
            spans.push((entity.coords, Self::entity_span(entity)));
            used_coords.push(entity.coords);
        }

        for y in MIN_COORDS.1..=MAX_COORDS.1 {
            for x in MIN_COORDS.0..=MAX_COORDS.0 {
                let mut has_object = false;
                for z in MIN_COORDS.2..=self.cursor.coords.2 {
                    if used_coords.contains(&Coords3D(x, y, z)) {
                        has_object = true;
                    }
                }
                if !has_object {
                    spans.push((Coords3D(x, y, MIN_COORDS.2), Self::void_span()));
                }
            }
        }

        spans.sort_by(|(coords1, _), (coords2, _)| coords1.partial_cmp(coords2).unwrap());

        let mut index = 0;
        for (i, (coords, _)) in spans.iter().enumerate() {
            if &self.cursor.coords == coords {
                index = i;
            }
        }

        spans[index].1 = spans[index].1.clone().add_modifier(Modifier::REVERSED);

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
            .title("Chunk Editor")
            .padding(Padding::top(((area.height - (MAX_COORDS.1 - MIN_COORDS.1) as u16) / 2) - 1));
        let p = Paragraph::new(lines).block(block).centered();

        frame.render_widget(p, area);

        if self.state == ChunkEditorState::EntityView {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Max(7),
                    Constraint::Length(120),
                    Constraint::Max(7),
                ])
                .split(area);

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Max(14),
                    Constraint::Length(60),
                    Constraint::Max(7),
                ])
                .split(layout[1]);

            return self.entity_view.render(frame, world, layout[1])
        }

        Ok(())
    }

    fn refresh_rate(&self) -> u16 {
        if self.state == ChunkEditorState::EntityView {
            return self.entity_view.refresh_rate();
        }
        
        60
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        scheme: ControlSchemeType,
        world: &World,
    ) -> Option<UiCallbackPreset> {
        if self.state == ChunkEditorState::EntityView {
            return self.entity_view.handle_key_events(key_event, scheme, world);
        }

        let keys = match scheme.keys_from_code(key_event.code) {
            Some(keys) => keys,
            None => return None,
        };

        for key in keys {
            match key {
                UiKey::MoveNorth => return Some(UiCallbackPreset::MoveChunkEditorCursor(Direction3D::North)),
                UiKey::MoveEast => return Some(UiCallbackPreset::MoveChunkEditorCursor(Direction3D::East)),
                UiKey::MoveWest => return Some(UiCallbackPreset::MoveChunkEditorCursor(Direction3D::West)),
                UiKey::MoveSouth => return Some(UiCallbackPreset::MoveChunkEditorCursor(Direction3D::South)),
                UiKey::MoveUp => return Some(UiCallbackPreset::MoveChunkEditorCursor(Direction3D::Up)),
                UiKey::MoveDown => return Some(UiCallbackPreset::MoveChunkEditorCursor(Direction3D::Down)),
                UiKey::EditEntity => return Some(UiCallbackPreset::EditEntity(self.cursor.coords)),
                UiKey::Save => return Some(UiCallbackPreset::SaveToDisk),
                _ => {},
            }
        }

        None
    }
}
