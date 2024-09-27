use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::world::world::World;

use super::{control_scheme::ControlSchemeType, traits::Screen, ui_callback::UiCallbackPreset};

pub struct OverworldWindow;

impl OverworldWindow {
    pub fn new() -> Self {
        OverworldWindow {}
    }
}

impl Screen for OverworldWindow {
    fn update(&mut self, world: &World) -> Result<()> {}

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {}

    fn get_refresh_rate(&self) -> u16 {
        60
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        scheme: ControlSchemeType,
        world: &World,
    ) -> Option<UiCallbackPreset> {
    }
}
