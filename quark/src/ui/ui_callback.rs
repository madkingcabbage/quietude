use std::path::PathBuf;

use anyhow::Result;
use quietude::{
    constants::SAVE_EXTENSION,
    types::{Coords3D, Direction1D, Direction3D, FormattedString, FormattedText},
    world::{
        entity::{EntityAttribute, EntityAttributeChoice, EntityAttributeText},
        log::LogStyle,
    },
};

use crate::{app::App, store::save_project, types::Message};

use super::{
    choice_menu::ChoiceMenu, chunk_editor::ChunkEditorState, dialogue_editor::DialogueEditorState, entity_view::EntityViewState, popup_message::PopupStyle, ui::{Ui, UiState}
};

pub enum UiCallbackPreset {
    MoveChunkEditorCursor(Direction3D),
    MoveEntityViewCursor(Direction1D),
    MoveChoiceMenuCursor(Direction1D),
    EditEntity(Coords3D),
    EditEntityAttribute(EntityAttribute, FormattedString<LogStyle>),
    ExitStringEditor(EntityAttributeText, String),
    ChoiceMenuSelectAndExit(String),
    ExitChoiceMenu,
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
            UiCallbackPreset::MoveChoiceMenuCursor(direction) => {
                app.ui.choice_menu.move_cursor(*direction)
            }
            UiCallbackPreset::EditEntity(coords) => app
                .ui
                .chunk_editor
                .edit_entity(*coords, &app.world.active_chunk)?,
            UiCallbackPreset::EditEntityAttribute(attr, default) => match attr {
                EntityAttribute::Text(attr) => app
                    .ui
                    .chunk_editor
                    .entity_view
                    .start_attr_text_editor(attr.clone(), default),
                EntityAttribute::Choice(attr) => {
                    let choices = attr.choices();
                    let cb = |s: &str, ui: &mut Ui| {
                        let attr = ui.chunk_editor.entity_view.get_current_attribute()?;
                        if let EntityAttribute::Choice(attr) = attr {
                            ui.chunk_editor.entity_view.set_choice_attr(attr.clone(), s)?;
                        }
                        Ok(())
                    };
                    app.ui.choice_menu =
                        ChoiceMenu::new(choices.iter().map(|s| String::from(*s)).collect(), cb);
                    app.ui.state = UiState::ChoiceMenu;
                }
            },
            UiCallbackPreset::ExitStringEditor(attr, s) => {
                app.ui
                    .chunk_editor
                    .entity_view
                    .set_text_attribute(attr.clone(), s);
                app.ui.chunk_editor.entity_view.state = EntityViewState::Main;
            }
            UiCallbackPreset::ChoiceMenuSelectAndExit(s) => {
                let s = &app.ui.choice_menu.options[app.ui.choice_menu.index].clone();
                (app.ui.choice_menu.on_exit.as_ref().unwrap().clone())(s, &mut app.ui)?;
                app.ui.state = UiState::Chunk;
            }
            UiCallbackPreset::ExitChoiceMenu => {
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
                let mut path = app.project_dir.to_path_buf();

                path.push(format!("{}{}", &tree.speaker_name, SAVE_EXTENSION));

                tree.save(PathBuf::from(path))?;
            }
            UiCallbackPreset::CloseUiPopup => app.ui.close_popup(),
            UiCallbackPreset::SaveToDisk => save_project(app)?,
        }

        Ok(None)
    }
}
