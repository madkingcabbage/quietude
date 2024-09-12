use anyhow::{anyhow, bail, Result};

use crate::{app::App, types::{Coords3D, Direction3D, Message}, world::{constants::PLAYER_ID, entity::EntityType}};

pub enum UiCallbackPreset {
    None,
    CloseUiPopup,
    SetSavename(String),
    MovePlayer(Direction3D),
    MoveCursor(Direction3D),
    InspectEntity(u32),
    InteractWithEntity(u32),
    Wait(u32),
}

impl UiCallbackPreset {
    pub fn call(&self, app: &mut App) -> Result<Option<Message>> {
        match self {
            UiCallbackPreset::CloseUiPopup => {
                app.ui.close_popup();
                Ok(None)
            }
            UiCallbackPreset::SetSavename(text) => {
                app.world.savename = Some(*text);
                Ok(None)
            }
            UiCallbackPreset::None =>
                Ok(None),
            UiCallbackPreset::MoveCursor(direction) => {
                app.ui.move_cursor(*direction);
                Ok(None)
            }
            UiCallbackPreset::MovePlayer(direction) => {
                app.world.pass_tick = true;
                app.world.active_chunk.move_entity(PLAYER_ID, direction)
            }
            UiCallbackPreset::InspectEntity(id) =>
                app.world.active_chunk.inspect_entity(id),
            UiCallbackPreset::InteractWithEntity(id) => {
                app.world.pass_tick = true;
                if let Some(player_coords) = app.world.active_chunk.get_entity_coords(PLAYER_ID) {
                    app.world.active_chunk.entity_interaction(PLAYER_ID, id)
                }
                Err(anyhow!("could not find player in chunk"))
            }
            UiCallbackPreset::Wait(ticks) =>
                app.world.pass_time(*ticks),
        }
    }
}
