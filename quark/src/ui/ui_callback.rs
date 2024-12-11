use std::path::PathBuf;

use anyhow::Result;
use quietude::{
    types::{Coords3D, Direction1D, Direction3D, FormattedString, FormattedText},
    world::{
        entity::{EntityAttribute, EntityAttributeText},
        log::LogStyle,
    },
};

use crate::{app::App, store::save_project, types::Message};

use super::{
    chunk_editor::ChunkEditorState, dialogue_editor::DialogueEditorState,
    entity_view::EntityViewState, popup_message::PopupStyle,
};

pub enum UiCallbackPreset {
    MoveChunkEditorCursor(Direction3D),
    MoveEntityViewCursor(Direction1D),
    MoveChoiceMenuCursor(Direction1D),
    EditEntity(Coords3D),
    EditEntityAttribute(EntityAttribute, FormattedString<LogStyle>),
    ExitStringEditor(EntityAttributeText, String),
    ExitChoiceMenu(String),
    ExitEntityView,
    MoveDialogueEditorCursor(Direction1D),
    ExitDialogueEditor,
    CloseUiPopup,
    SaveToDisk,
}

impl UiCallbackPreset {
    pub fn call(&self, app: &mut App) -> Result<Option<Message>> {
        match self {
            UiCallbackPreset::MoveChunkEditorCursor(direction) => app
                .ui
                .chunk_editor
                .cursor
                .coords
                .move_in_direction(direction),
            UiCallbackPreset::MoveEntityViewCursor(direction) => {
                app.ui.chunk_editor.entity_view.move_cursor(direction)
            }
            UiCallbackPreset::MoveChoiceMenuCursor(direction) => app
                .ui
                .chunk_editor
                .entity_view
                .choice_attr_editor
                .menu
                .move_cursor(*direction),
            UiCallbackPreset::EditEntity(coords) => app
                .ui
                .chunk_editor
                .edit_entity(*coords, &app.world.active_chunk)?,
            UiCallbackPreset::EditEntityAttribute(attr, default) => app
                .ui
                .chunk_editor
                .entity_view
                .start_attribute_editor(attr.clone(), default),
            UiCallbackPreset::ExitStringEditor(attr, s) => {
                app.ui
                    .chunk_editor
                    .entity_view
                    .set_text_attribute(attr.clone(), s);
                app.ui.chunk_editor.entity_view.state = EntityViewState::Main;
            }
            UiCallbackPreset::ExitChoiceMenu(s) => {
                let attr = app
                    .ui
                    .chunk_editor
                    .entity_view
                    .choice_attr_editor
                    .attr
                    .clone();
                app.ui.chunk_editor.entity_view.set_choice_attr(attr, s)?;
                app.ui.chunk_editor.entity_view.state = EntityViewState::Main;
            }
            UiCallbackPreset::ExitEntityView => {
                let entity = app.ui.chunk_editor.entity_view.finish()?;
                let coords = entity.coords.clone();
                app.next_valid_entity_id = app.world.active_chunk.overwrite_entity(
                    entity,
                    &coords,
                    app.next_valid_entity_id,
                );
                app.ui.chunk_editor.state = ChunkEditorState::Main;
            }
            UiCallbackPreset::MoveDialogueEditorCursor(direction) => {
                app.ui.dialogue_editor.move_highlight(direction)?
            }
            UiCallbackPreset::ExitDialogueEditor => {
                app.ui.dialogue_editor.state = DialogueEditorState::Main;
                let tree = app.ui.dialogue_editor.finish()?;
                if app.project_dir.is_none() {
                    return Ok(Some(Message::Popup(FormattedString::from(
                        &None,
                        FormattedText::new(
                            "Choose a directory for the project before saving",
                            PopupStyle::Error,
                        ),
                    ))));
                }

                let mut path = app.project_dir.clone().unwrap();

                path.push_str(&tree.speaker_name);

                tree.save(PathBuf::from(path))?;
            }
            UiCallbackPreset::CloseUiPopup => app.ui.close_popup(),
            UiCallbackPreset::SaveToDisk => save_project(app)?,
        }

        Ok(None)
    }
}
