use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    rng::TickBasedRng,
    types::{Coords3D, Message},
};

use super::{
    action::Action,
    entity::{Entity, Focus},
};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Chunk {
    entities: HashMap<u32, Entity>,
}

impl Chunk {
    pub fn from_seed(coords: Coords3D, seed: u32, next_entity_id: &mut u32) -> Result<Self> {
        Ok(Default::default())
    }

    pub fn pass_tick(&mut self, rng: &mut TickBasedRng) -> Result<Vec<Message>> {
        let mut messages = vec![];
        let mut ids = vec![];
        for (id, _) in &self.entities {
            ids.push(*id);
        }
        for id in &ids {
            if let Some(entity) = self.get_entity_from_id(*id) {
                if entity.has_agency {
                    if let Some(message) = self.npc_turn(*id, rng)? {
                        messages.push(message);
                    }
                }
            }
        }
        Ok(messages)
    }

    pub fn get_entities(&self) -> &HashMap<u32, Entity> {
        &self.entities
    }

    pub fn get_entity_from_id(&self, id: u32) -> Option<&Entity> {
        self.entities.get(&id)
    }

    pub fn get_entity_mut_from_id(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }

    pub fn get_entity_from_coords(&self, coords: &Coords3D) -> Option<&Entity> {
        for entity in &self.entities {
            if entity.1.coords == *coords {
                return Some(entity.1)
            }
        }
        None
    }

    pub fn get_entity_coords(&self, id: u32) -> Option<&Coords3D> {
        let result = &self.entities.get(&id);
        if result.is_none() {
            None
        } else {
            Some(&result.unwrap().coords)
        }
    }

    fn get_entity_mut_from_coords(&mut self, coords: &Coords3D) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if entity.1.coords == *coords {
                return Some(entity.1)
            }
        }
        None
    }

    fn npc_turn(&mut self, id: u32, rng: &mut TickBasedRng) -> Result<Option<Message>> {
        let mut entity = self.get_entity_mut_from_id(id).ok_or(anyhow!(
            "entity could not be found from id: {id}",
        ))?;

        if entity.focus.is_none() {
            entity.find_new_focus(id, self, rng)
        }

        let Focus { action: focus, ..} = entity.focus.unwrap();
        if let Action::Solo(solo_action) = focus {
            self.entity_take_solo_action(id, solo_action)?
        } else {
            self.entity_interaction(id, focus)?
        }
    }
}
