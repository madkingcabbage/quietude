use std::default;

use anyhow::Result;
use crossterm::event::KeyEvent;
use log::error;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::{
    constants::{MAX_COORDS, MIN_COORDS},
    world::world::World,
};

use super::{
    control_scheme::ControlSchemeType, dialogue_window::DialogueWindow, log_window::LogWindow,
    overworld_window::OverworldWindow, traits::Screen, ui_callback::UiCallbackPreset,
};

pub struct MainScreen {
    state: ScreenState,
    pub overworld_window: OverworldWindow,
    pub dialogue_window: DialogueWindow,
    pub log_window: LogWindow,
}

#[derive(Default)]
pub enum ScreenState {
    #[default]
    Overworld,
    Dialogue,
    Log,
}

impl MainScreen {
    pub const OVERWORLD_WIDTH_MIN: u16 = (4 + MAX_COORDS.0 + -MIN_COORDS.0) as u16;
    pub const OVERWORLD_HEIGHT_MIN: u16 = (4 + MAX_COORDS.1 + -MIN_COORDS.1) as u16;
    pub const LOG_WIDTH_MIN: u16 = 80;
    pub const LOG_HEIGHT: u16 = 16;
    pub const DIALOGUE_WIDTH: u16 = 36;

    pub fn new() -> Self {
        MainScreen {
            state: ScreenState::default(),
            overworld_window: OverworldWindow::new(),
            dialogue_window: DialogueWindow::new(),
            log_window: LogWindow::new(),
        }
    }

    pub fn get_active_window_mut(&mut self) -> &mut dyn Screen {
        match self.state {
            ScreenState::Overworld => &mut self.overworld_window,
            ScreenState::Dialogue => &mut self.dialogue_window,
            ScreenState::Log => &mut self.log_window,
        }
    }

    pub fn get_active_window(&self) -> &dyn Screen {
        match self.state {
            ScreenState::Overworld => &self.overworld_window,
            ScreenState::Dialogue => &self.dialogue_window,
            ScreenState::Log => &self.log_window,
        }
    }
}

impl Screen for MainScreen {
    fn update(&mut self, world: &World) -> Result<()> {
        self.overworld_window.update(world)?;
        self.dialogue_window.update(world)?;
        self.log_window.update(world)?;
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
        let screen = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Min(Self::LOG_WIDTH_MIN.max(Self::OVERWORLD_WIDTH_MIN)),
                Constraint::Length(Self::DIALOGUE_WIDTH),
            ])
            .split(area);

        let overworld_log_chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Min(Self::OVERWORLD_HEIGHT_MIN),
                Constraint::Length(Self::LOG_HEIGHT),
            ])
            .split(screen[0]);

        self.dialogue_window
            .render(frame, world, screen[1])
            .unwrap_or_else(|e| error!("{} while rendering dialogue window", e.to_string()));
        self.overworld_window
            .render(frame, world, overworld_log_chunks[0])
            .unwrap_or_else(|e| error!("{} while rendering log window", e.to_string()));
        self.log_window
            .render(frame, world, overworld_log_chunks[1])
            .unwrap_or_else(|e| error!("{} while rendering log window", e.to_string()));

        Ok(())
    }

    fn refresh_rate(&self) -> u16 {
        self.get_active_window().refresh_rate()
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        scheme: ControlSchemeType,
        world: &World,
    ) -> Option<UiCallbackPreset> {
        self.get_active_window_mut()
            .handle_key_events(key_event, scheme, world)
    }
}
