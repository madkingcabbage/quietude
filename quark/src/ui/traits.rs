use std::fmt::{Debug, Formatter};

use anyhow::Result;
use crossterm::event::KeyEvent;
use quietude::world::world::World;
use ratatui::{layout::Rect, Frame};

use super::{control_scheme::ControlSchemeType, ui_callback::UiCallbackPreset};

pub trait Screen {
    fn update(&mut self, world: &World) -> Result<()>;

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()>;

    fn handle_key_events(&mut self, key_event: KeyEvent, scheme: ControlSchemeType, world: &World)
        -> Option<UiCallbackPreset>;

    fn refresh_rate(&self) -> u16;
}

impl Debug for dyn Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Screen {:?}", self)
    }
}
