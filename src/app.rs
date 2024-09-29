use std::{io, time::Duration};

use anyhow::{Context, Result};
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent, MouseEvent};
use log::{error, info};
use ratatui::{prelude::CrosstermBackend, Frame};

use crate::{
    store::save,
    types::{FormattedString, FormattedText, Message},
    ui::{
        popup_message::{PopupMessage, PopupStyle},
        tui::Tui,
        ui::Ui,
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
        let mut tui =
            Tui::new().unwrap_or_else(|e| panic!("{} while initializing terminal", e.to_string()));
        while self.running {
            if poll(frequency_to_period(
                self.ui.get_current_refresh_rate() as u32
            ))
            .unwrap_or_else(|e| panic!("{} while polling", e.to_string()))
            {
                match event::read()
                    .unwrap_or_else(|e| panic!("{} while reading events", e.to_string()))
                {
                    Event::Key(key) => self.handle_key_events(key).unwrap_or_else(|e| {
                        error!("{} while handling key event {:?}", e.to_string(), key)
                    }),
                    //                    Event::Mouse(mouse) => self.handle_mouse_events(mouse)?,
                    _ => {}
                }
            } else {
                self.update()
                    .unwrap_or_else(|e| error!("{} during update", e.to_string()));
            }
            tui.draw(&mut self.ui, &self.world)
                .unwrap_or_else(|e| error!("{} while drawing to terminal", e.to_string()));
        }

        Tui::restore().unwrap_or_else(|e| panic!("{} while restoring terminal", e.to_string()));
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

    pub fn render(ui: &mut Ui, world: &World, frame: &mut Frame) -> Result<()> {
        ui.render(frame, world)?;
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.ui.update(&self.world)
    }

    fn quit(&mut self) -> Result<()> {
        if self.world.savename.is_some() {
            save(&self.world.savename.clone().unwrap(), &self.world)?;
        }
        self.running = false;
        Ok(())
    }
}
