use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::{types::FormattedString, world::world::World};

use super::{control_scheme::ControlSchemeType, main_screen::MainScreen, traits::Screen, ui_callback::UiCallbackPreset};

pub struct LogWindow;

impl LogWindow {
    pub fn new() -> Self {
        LogWindow {}
    }
}

impl Screen for LogWindow {
    fn update(&mut self, _world: &World) -> Result<()> {
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame, world: &World, area: Rect) -> Result<()> {
        let mut lines = vec![];
        for string in &world.log.contents {
            lines.push(Line::from(FormattedString::into_spans(string)));
        }

        let block = Block::default().borders(Borders::ALL).title("Log");
        let p = Paragraph::new(lines).block(block);

        frame.render_widget(p, area);
        Ok(())
    }

    fn refresh_rate(&self) -> u16 {
        60
    }

    // TODO: scrolling
    fn handle_key_events(
        &mut self,
        _key_event: KeyEvent,
        _scheme: ControlSchemeType,
        _world: &World,
    ) -> Option<UiCallbackPreset> {
        None
    }
}
