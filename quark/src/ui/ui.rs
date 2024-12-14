use anyhow::Result;
use crossterm::event::KeyEvent;
use quietude::{types::{Direction1D, Direction3D}, world::world::World};
use ratatui::Frame;
use tui_textarea::TextArea;

use super::{choice_menu::ChoiceMenu, chunk_editor::ChunkEditor, control_scheme::ControlSchemeType, dialogue_editor::DialogueEditor, popup_message::PopupMessage, traits::Screen, ui_callback::UiCallbackPreset};

pub struct Ui {
    pub state: UiState,
    pub chunk_editor: ChunkEditor,
    pub dialogue_editor: DialogueEditor,
    popup_input: TextArea<'static>,
    popup_messages: Vec<PopupMessage>,
    pub scheme: ControlSchemeType,
    pub choice_menu: ChoiceMenu,
}

#[derive(Default)]
pub enum UiState {
    #[default]
    Chunk,
    Dialogue,
    ChoiceMenu,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            state: UiState::default(),
            chunk_editor: ChunkEditor::new(),
            dialogue_editor: DialogueEditor::new(),
            popup_input: TextArea::default(),
            popup_messages: vec![],
            scheme: ControlSchemeType::default(),
            choice_menu: ChoiceMenu::default(),
        }
    }

    pub fn set_popup(&mut self, popup_message: PopupMessage) {
        self.popup_messages.push(popup_message);
    }

    pub fn close_popup(&mut self) {
        self.popup_messages.remove(0);
    }

    pub fn move_cursor(&mut self, direction: &Direction3D) {
        self.chunk_editor.cursor.coords.move_in_direction(direction);
    }

    pub fn move_dialogue_highlight(&mut self, direction: &Direction1D) -> Result<()> {
        self.dialogue_editor.move_highlight(direction)?;

        Ok(())
    }

    pub fn handle_key_events(&mut self, key: KeyEvent, world: &World) -> Option<UiCallbackPreset> {
        let scheme = self.scheme;
        if self.popup_messages.len() > 0 {
            return self.popup_messages[0].consumes_input(&mut self.popup_input, key, &scheme);
        }

        self.get_active_screen_mut().handle_key_events(key, scheme, world)
    }

    fn get_active_screen_mut(&mut self) -> &mut dyn Screen {
        match self.state {
            UiState::Chunk => &mut self.chunk_editor,
            UiState::Dialogue => &mut self.dialogue_editor,
            UiState::ChoiceMenu => &mut self.choice_menu,
        }
    }

    fn get_active_screen(&self) -> &dyn Screen {
        match self.state {
            UiState::Chunk => &self.chunk_editor,
            UiState::Dialogue => &self.dialogue_editor,
            UiState::ChoiceMenu => &self.choice_menu,
        }
    }

    pub fn get_current_refresh_rate(&self) -> u16 {
        self.get_active_screen().refresh_rate()
    }

    pub fn update(&mut self, world: &World) -> Result<()> {
        self.get_active_screen_mut().update(world)
    }

    pub fn render(&mut self, f: &mut Frame, world: &World) -> Result<()> {
        self.get_active_screen_mut().render(f, world, f.area())
    }
}
