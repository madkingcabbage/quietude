use std::rc::Rc;

use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use tui_textarea::TextArea;

use crate::{
    types::{Coords3D, Direction1D, Direction3D},
    world::world::World,
};

use super::{
    control_scheme::ControlSchemeType, cursor::Cursor, main_screen::MainScreen,
    popup_message::PopupMessage, splash_screen::SplashScreen, traits::Screen,
    ui_callback::UiCallbackPreset,
};

pub struct Ui {
    pub state: UiState,
    pub splash_screen: SplashScreen,
    pub main_screen: MainScreen,
    popup_input: TextArea<'static>,
    popup_messages: Vec<PopupMessage>,
    pub scheme: ControlSchemeType,
}

#[derive(Default, PartialEq, Eq)]
pub enum UiState {
    #[default]
    Splash,
    Main,
}

pub enum UiWindow {
    Main,
    Dialogue,
    Log,
}

impl Ui {
    pub fn new() -> Self {
        let ui_windows = vec![UiWindow::Main, UiWindow::Dialogue, UiWindow::Log];
        Self {
            state: UiState::default(),
            splash_screen: SplashScreen::new(),
            main_screen: MainScreen::new(),
            popup_input: TextArea::default(),
            popup_messages: vec![],
            scheme: ControlSchemeType::default(),
        }
    }

    pub fn set_popup(&mut self, popup_message: PopupMessage) {
        self.popup_messages.push(popup_message);
    }

    pub fn close_popup(&mut self) {
        self.popup_messages.remove(0);
    }

    pub fn move_cursor(&mut self, direction: &Direction3D) {
        self.main_screen.overworld_window.cursor.coords.move_in_direction(direction);
    }

    pub fn move_dialogue_highlight(&mut self, direction: &Direction1D, max_choice: usize) {
        self.main_screen.dialogue_window.move_highlight(*direction, max_choice);
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
            UiState::Splash => &mut self.splash_screen,
            UiState::Main => &mut self.main_screen,
        }
    }

    fn get_active_screen(&self) -> &dyn Screen {
        match self.state {
            UiState::Splash => &self.splash_screen,
            UiState::Main => &self.main_screen,
        }
    }

    pub fn get_current_refresh_rate(&self) -> u16 {
        self.get_active_screen().get_refresh_rate()
    }

    pub fn update(&mut self, world: &World) -> Result<()> {
        self.get_active_screen_mut().update(world)
    }

    pub fn render(&mut self, f: &mut Frame, world: &World) -> Result<()> {
        self.get_active_screen_mut().render(f, world, f.area())
    }
}
