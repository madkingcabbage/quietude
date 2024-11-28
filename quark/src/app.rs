use std::io;

use anyhow::Result;
use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use log::error;
use quietude::{
    types::{FormattedString, FormattedText},
    utils::frequency_to_period,
    world::{chunk::Chunk, world::World},
};
use ratatui::{prelude::CrosstermBackend, Frame};

use crate::{
    store::{guarantee_project_structure, load_project, save_project}, types::Message, ui::{
        popup_message::{PopupMessage, PopupStyle},
        tui::Tui,
        ui::Ui,
    }
};

pub struct App {
    pub world: World,
    pub next_valid_entity_id: u32,
    pub running: bool,
    pub ui: Ui,
    pub project_dir: Option<String>,
}

impl App {
    pub fn new(project_dir: Option<String>) -> Self {
        let (id, chunk) = if let Some(ref project_dir) = project_dir {
            if guarantee_project_structure(&project_dir).unwrap_or_else(|e| panic!("{e} while checking project directory structure")) {
                load_project(&project_dir).unwrap_or_else(|e| panic!("{e} while loading project"))
            } else {
                (0, Chunk::default())
            }
        } else {
            (0, Chunk::default())
        };

        let mut world =
            World::from_seed(0).unwrap_or_else(|e| panic!("{e} while generating world"));
        world
            .add_chunk_in_dir(project_dir.as_ref().unwrap(), chunk)
            .unwrap_or_else(|e| panic!("{e} while giving chunk to world"));

        App {
            world,
            next_valid_entity_id: id,
            running: true,
            ui: Ui::new(),
            project_dir: project_dir.clone(),
        }
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

    pub fn render(ui: &mut Ui, world: &World, frame: &mut Frame) -> Result<()> {
        ui.render(frame, world)?;
        Ok(())
    }

    fn handle_key_events(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
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

    fn update(&mut self) -> Result<()> {
        self.ui.update(&self.world)
    }

    fn quit(&mut self) -> Result<()> {
        save_project(&self)?;
        self.running = false;
        Ok(())
    }
}
