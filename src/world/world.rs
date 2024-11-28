use std::{fmt::Debug, fs::{DirBuilder, File}, path::{Path, PathBuf}, u32};

use anyhow::{anyhow, Result};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    constants::{SAVE_EXTENSION, WORLD_DIR_NAME}, rng::TickBasedRng, store::{load_profile, save, save_profile}, types::{Coords3D, Coords4D, Message}
};

use super::{
    chunk::Chunk,
    conditions::{self, WorldCondition},
    dialogue::{DialogueOutcome, DialogueTree},
    log::Log,
};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    seed: u32,
    pub savename: Option<String>,
    pub active_chunk: Chunk,
    pub chunk_coords: Coords4D,
    tick: u32,
    rng: TickBasedRng,
    pub pass_tick: bool,
    pub messages: Vec<Message>,
    next_entity_id: u32,
    pub log: Log,
    pub dialogue_tree: Option<DialogueTree>,
    conditions: Vec<WorldCondition>,
}

impl World {
    pub fn new(savename: &str) -> Result<Self> {
        let world = World::from_savename(savename);
        if world.is_ok() {
            info!("Loading world from save: {savename}.");
            world
        } else {
            info!("Could not load save {savename}. Creating new save.");
            Self::from_seed(rand::random())
        }
    }

    pub fn from_seed(seed: u32) -> Result<Self> {
        info!("Generating new world from seed: {seed}.");
        let mut next_entity_id = u32::MAX / 2;
        let active_chunk = Chunk::from_seed(Coords4D(0, 0, 0, 0), seed, &mut next_entity_id)?;
        Ok(World {
            seed,
            savename: None,
            active_chunk,
            chunk_coords: Coords4D(0, 0, 0, 0),
            tick: 0,
            rng: TickBasedRng::new(seed, 0),
            messages: vec![],
            pass_tick: false,
            next_entity_id,
            log: Log::new(),
            dialogue_tree: None,
            conditions: Vec::new(),
        })
    }

    pub fn add_chunk(&mut self, chunk: Chunk) -> Result<()> {
        let filename = format!(
            "world/{}{SAVE_EXTENSION}",
            self.chunk_coords
        );
        info!("saving chunk to {filename}");
        save_profile(&filename, &self.active_chunk)?;
        
        self.active_chunk = chunk;
        Ok(())
    }

    pub fn add_chunk_in_dir(&mut self, project_dir: &str, chunk: Chunk) -> Result<()> {
        let filename = format!(
            "{project_dir}{WORLD_DIR_NAME}{}{SAVE_EXTENSION}",
            self.chunk_coords
        );
        info!("saving chunk to {filename}");
        save(&PathBuf::from(&filename), &self.active_chunk)?;
        
        self.active_chunk = chunk;
        Ok(())
    }

    fn from_savename(savename: &str) -> Result<Self> {
        let mut filename = String::from(savename);
        filename.push_str(".json");

        load_profile(&filename)
    }

    pub fn change_to_chunk(&mut self, coords: Coords4D) -> Result<()> {
        let filename = format!(
            "world/{coords}.json",
        );
        info!("saving chunk to {filename}");
        save_profile(&filename, &self.active_chunk)?;

        let filename = format!("world/{}.json", coords.to_str());
        self.active_chunk = if let Ok(chunk_try) = load_profile(&filename) {
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
            for message in self.active_chunk.pass_tick(&mut self.rng, self.tick)? {
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
        save_profile(&savename, &world_clone);
    }

    pub fn has_condition(&self, condition: WorldCondition) -> bool {
        for world_condition in &self.conditions {
            if *world_condition == condition {
                return true;
            }
        }

        false
    }

    pub fn add_condition(&mut self, condition: WorldCondition) {
        self.conditions.push(condition);
    }

    pub fn remove_condition(&mut self, condition: WorldCondition) {
        let mut should_remove = false;
        let mut remove_index = 0;
        for (i, world_condition) in self.conditions.iter().enumerate() {
            if *world_condition == condition {
                should_remove = true;
                remove_index = i;
                break;
            }
        }

        if should_remove {
            self.conditions.remove(remove_index);
        }
    }

    fn activate_dialogue_outcome(&mut self, outcome: &DialogueOutcome) -> Result<()> {
        let interlocutor_id = self.dialogue_tree.as_ref().unwrap().interlocutor_id;
        let speaker_id = self.dialogue_tree.as_ref().unwrap().speaker_id;
        match outcome {
            DialogueOutcome::GiveInterlocutorItem(item) => {
                self
                    .active_chunk
                    .get_entity_mut_from_id(interlocutor_id)
                    .ok_or(anyhow!("interlocutor {} not found", interlocutor_id))?
                    .give_item(item.clone())?;
            }
            DialogueOutcome::GiveInterlocutorSpecificItem(id) => {
                self
                    .active_chunk
                    .get_entity_mut_from_id(interlocutor_id)
                    .ok_or(anyhow!("interlocutor {} not found", interlocutor_id))?
                    .give_specific_item(*id)?;
            }
            DialogueOutcome::TakeInterlocutorSpecificItem(id) => {
                self
                    .active_chunk
                    .get_entity_mut_from_id(interlocutor_id)
                    .ok_or(anyhow!("interlocutor {} not found", interlocutor_id))?
                    .take_specific_item(*id)?;
            }
            DialogueOutcome::AddWorldCondition(condition) => {
                self.add_condition(*condition);
            }
            DialogueOutcome::RemoveWorldCondition(condition) => {
                self.remove_condition(*condition);
            }
        };

        Ok(())
    }

    pub fn begin_dialogue(&mut self, speaker_id: u32, interlocutor_id: u32) -> Result<()> {
        self.dialogue_tree = Some(DialogueTree::from_entity_name(speaker_id, interlocutor_id)?);
        Ok(())
    }

    pub fn make_dialogue_choice(&mut self, index: usize) -> Result<()> {
        let (outcomes, destination) = self.dialogue_tree
            .as_ref()
            .unwrap_or(Err(anyhow!("no dialogue tree currently active"))?)
            .get_outcomes_and_destination_from_choice(index, self)?;

        for outcome in &outcomes {
            self.activate_dialogue_outcome(outcome)?;
        }

        self.dialogue_tree.as_mut().unwrap_or(Err(anyhow!("no dialogue tree currently active"))?)
            .make_choice(&destination);
        Ok(())
    }
}
