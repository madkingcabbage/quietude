use anyhow::{anyhow, Result};

use crate::{
    app::App,
    types::{Direction1D, Direction3D, Message},
    world::constants::PLAYER_ID,
};

pub enum UiCallbackPreset {
    None,
    CloseUiPopup,
    SetSavename(String),
    MovePlayer(Direction3D),
    MoveCursor(Direction3D),
    MoveDialogueHighlight(Direction1D),
    SelectDialogueEntry(usize),
    InspectEntity(u32),
    TalkToEntity(u32),
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
                app.world.savename = Some(text.clone());
                Ok(None)
            }
            UiCallbackPreset::None => Ok(None),
            UiCallbackPreset::MoveCursor(direction) => {
                app.ui.move_cursor(direction);
                Ok(None)
            }
            UiCallbackPreset::MoveDialogueHighlight(direction) => {
                let max_choice = app.world
                    .dialogue_tree
                    .as_ref()
                    .unwrap_or(Err(anyhow!("active tree not found in world"))?)
                    .get_active_node()?
                    .choices_text(&app.world)?
                    .len();
        
                app.ui.move_dialogue_highlight(direction, max_choice);
                Ok(None)
            }
            UiCallbackPreset::SelectDialogueEntry(index) => {
                app.world.make_dialogue_choice(*index)?;
                Ok(None)
            }
            UiCallbackPreset::MovePlayer(direction) => {
                app.world.pass_tick = true;
                app.world.active_chunk.move_entity(PLAYER_ID, direction);
                Ok(None)
            }
            UiCallbackPreset::InspectEntity(id) => {
                let message = app.world.active_chunk.inspect_entity(*id)?;
                Ok(Some(Message::Log(message)))
            }
            UiCallbackPreset::TalkToEntity(id) => {
                app.world.pass_tick = true;
                app.world.begin_dialogue(*id, PLAYER_ID)?;
                Ok(None)
            }
            UiCallbackPreset::Wait(ticks) => {
                app.world.pass_time(*ticks);
                Ok(None)
            }
        }
    }
}
