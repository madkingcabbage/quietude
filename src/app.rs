use std::{io, time::Duration};

use anyhow::Result;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, MouseEvent};
use log::info;
use ratatui::prelude::CrosstermBackend;

use crate::{
    store::save,
    types::{FormattedString, FormattedText, Message},
    ui::{
        popup_message::{PopupMessage, PopupStyle}, tui::Tui, ui::Ui
    },
    utils::frequency_to_period,
    world::{log::LogStyle, world::World},
};

pub struct App {
    pub world: World,
    pub running: bool,
    pub ui: Ui,
}

impl App {
    pub fn new(seed: Option<u32>, savename: Option<String>) -> Result<Self> {
        let seed = if let Some(seed_try) = seed {
            seed_try
        } else {
            rand::random()
        };

        let world = if let Some(savename) = savename {
            World::new(&savename)?
        } else {
            World::from_seed(seed)?
        };

        Ok(App {
            world,
            running: true,
            ui: Ui::new(),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let writer = io::stdout();
        let _backend = CrosstermBackend::new(writer);
        let mut _tui = Tui::new()?;
        while self.running {
            if poll(frequency_to_period(
                self.ui.get_current_refresh_rate() as u32
            ))? {
                match event::read()? {
                    Event::Key(key) => self.handle_key_events(key)?,
//                    Event::Mouse(mouse) => self.handle_mouse_events(mouse)?,
                    _ => {}
                }
            } else {
                self.update()?;
            }
        }

        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.quit()?;
            }
            _ => {
                if let Some(callback) = self.ui.handle_key_events(key, &self.world) {
                    match callback.call(self) {
                        Ok(Some(Message::Popup(text))) => {
                            self.ui.set_popup(PopupMessage::Ok(text));
                        }
                        Ok(Some(Message::Log(text))) => {
                            self.world.log.print_formatted_string(text);
                        }
                        Ok(None) => {}
                        Err(e) => {
                            let string = FormattedString::from(
                                &None,
                                FormattedText::new(&e.to_string(), PopupStyle::Error),
                            );
                            self.ui.set_popup(PopupMessage::Err(string));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /*
    fn handle_mouse_events(&mut self, mouse: MouseEvent) -> Result<()> {
        if let Some(callback) = self.ui.handle_mouse_events(mouse, &self.world) {
            match callback.call(self) {
                Ok(Message::Popup(text)) => {
                    self.ui.set_popup(PopupMessage::Ok(text));
                }
                Ok(Message::Log(text)) => {
                    self.world.log.print_formatted_string(text);
                }
                Ok(None) => {}
                Err(e) => {
                    self.ui.set_popup(PopupMessage::Err(e.to_string()));
                }
            }
        }
        Ok(())
    }
    */

    fn update(&mut self) -> Result<()> {
        self.ui.update(&self.world)
    }

    fn quit(&mut self) -> Result<()> {
        save(&self.world.savename.clone().unwrap(), &self.world)?;
        Tui::restore()
    }
}
