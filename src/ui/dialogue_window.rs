use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::world::world::World;

use super::{traits::Screen, ui_callback::UiCallbackPreset};

pub struct DialogueWindow {
}

impl DialogueWindow {
    pub fn new() -> Self {
        DialogueWindow {}
    }
}

impl Screen for DialogueWindow {
    fn update(&mut self, world: &World) -> Result<()> {}

    fn render(
        &mut self,
        frame: &mut Frame,
        world: &World,
        area: Rect,
    ) -> Result<()> {
    }

    fn get_refresh_rate(&self) -> u16 {
        60
    }

    fn handle_key_events(
        &mut self,
        key_event: KeyEvent,
        world: &World,
    ) -> Option<UiCallbackPreset> {
    }
}
