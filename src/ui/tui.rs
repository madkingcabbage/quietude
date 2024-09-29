use std::io::{stdout, Stdout};

use anyhow::Result;
use crossterm::event::DisableMouseCapture;
use ratatui::{
    crossterm::{
        cursor,
        event::EnableMouseCapture,
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    Terminal,
};

pub struct Tui(Terminal<CrosstermBackend<Stdout>>);

impl Tui {
    pub fn new() -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        let mut tui = Self(terminal);
        tui.init()?;
        Ok(tui)
    }

    fn init(&mut self) -> Result<Self> {
        execute!(
            stdout(),
            cursor::Hide,
            EnterAlternateScreen,
            EnableMouseCapture,
        )?;
        enable_raw_mode()?;
        Ok(Self(Terminal::new(CrosstermBackend::new(stdout()))?))
    }

    pub fn restore() -> Result<()> {
        execute!(stdout(), cursor::Show, DisableMouseCapture, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
}
