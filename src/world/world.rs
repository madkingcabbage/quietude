use std::fmt::Debug;

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    rng::TickBasedRng,
    store::{load, save},
    types::{Coords3D, Message},
};

use super::{chunk::Chunk, log::Log};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    seed: u32,
    pub savename: Option<String>,
    pub active_chunk: Chunk,
    chunk_coords: Coords3D,
    tick: u32,
    rng: TickBasedRng,
    pub pass_tick: bool,
    pub messages: Vec<Message>,
    next_entity_id: u32,
    pub log: Log,
}

impl World {
    pub fn new(savename: &str) -> Result<Self> {
        let world = World::from_savename(savename);
        if world.is_ok() {
            world
        } else {
            Self::from_seed(rand::random())
        }
    }

    pub fn from_seed(seed: u32) -> Result<Self> {
        info!("Generating new world from seed: {seed}.");
        let mut next_entity_id = 0;
        let active_chunk = Chunk::from_seed(Coords3D(0, 0, 0), seed, &mut next_entity_id)?;
        Ok(World {
            seed,
            savename: None,
            active_chunk,
            chunk_coords: Coords3D(0, 0, 0),
            tick: 0,
            rng: TickBasedRng::new(seed, 0),
            messages: vec![],
            pass_tick: false,
            next_entity_id,
            log: Log::new(),
        })
    }

    fn from_savename(savename: &str) -> Result<Self> {
        info!("Loading world from save: {savename}.");

        let mut filename = String::from(savename);
        filename.push_str(".json");

        load(&filename)
    }

    pub fn change_to_chunk(&mut self, coords: Coords3D) -> Result<()> {
        let filename = format!(
            "world/{},{},{}.json",
            self.chunk_coords.0, self.chunk_coords.1, self.chunk_coords.2
        );
        info!("saving chunk to {filename}");
        save(&filename, &self.active_chunk)?;

        let filename = format!("world/{},{}.json", coords.0, coords.1);
        self.active_chunk = if let Ok(chunk_try) = load(&filename) {
            info!("loading chunk from {filename}");
            chunk_try
        } else {
            info!("generating new chunk at {:?} from {}", coords, self.seed);
            Chunk::from_seed(coords, self.seed, &mut self.next_entity_id)?
        };

        Ok(())
    }

    pub fn pass_time(&mut self, ticks: u32) -> Result<()> {
        for _ in 0..ticks {
            for message in self.active_chunk.pass_tick(&mut self.rng)? {
                self.messages.push(message);
            }
        }
        self.tick += ticks;
        self.rng.reset(self.tick);
        Ok(())
    }

    pub fn set_savename(&mut self, s: &str) {
        self.savename = Some(String::from(s));
    }

    pub fn save_world(&self) {
        let savename = self.savename.clone().unwrap_or(String::from("world"));
        let world_clone = World {
            seed: self.seed,
            savename: self.savename.clone(),
            active_chunk: self.active_chunk.clone(),
            chunk_coords: self.chunk_coords.clone(),
            tick: self.tick,
            next_entity_id: self.next_entity_id,
            ..Default::default()
        };
        save(&savename, &world_clone);
    }
}
