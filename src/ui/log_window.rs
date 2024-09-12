use anyhow::Result;
use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, style::Style, text::{Line, Span}, widgets::{Block, Borders, Paragraph, Widget}, Frame};

use crate::world::world::World;

use super::{main_screen::MainScreen, traits::Screen, ui_callback::UiCallbackPreset};

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
        let mut spans = vec![];
        for (s, style) in &world.log.contents {
            spans.push(Span::styled(s, style.clone()));
        }

        let block = Block::default().borders(Borders::ALL).title("Log");
        let p = Paragraph::new(vec![Line::from(spans)]).block(block);

        frame.render_widget(p, area);
        Ok(())
    }

    fn get_refresh_rate(&self) -> u16 {
        60
    }

    // TODO: scrolling
    fn handle_key_events(
        &mut self,
        _key_event: KeyEvent,
        _world: &World,
    ) -> Option<UiCallbackPreset> {
        None
    }
}
