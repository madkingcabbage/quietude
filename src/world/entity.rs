use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    rng::TickBasedRng,
    types::Coords3D,
    utils::{avg, insert_noise, product},
};

use super::{action::Action, chunk::Chunk, faction::Faction};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub enum EntityType {
    Player,
    Grass,
    Tree,
    NpcFriendly,
    #[default]
    Void,
}

pub static mut NEXT_ID: u32 = 0;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub name: String,
    pub description: String,
    pub can_move: bool,
    pub has_agency: bool,
    pub allegiance: Option<String>,
    pub focus: Option<Focus>,
    pub size: Size,
    pub coords: Coords3D,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Focus {
    pub action: Action,
    pub tick_born: u32,
}

pub enum Opinion {
    Love,
    Friendly,
    Neutral,
    Enemy,
    ArchNemesis,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Entity {
    pub fn find_new_focus(&mut self, id: u32, chunk: &Chunk, rng: &mut TickBasedRng) {
        let weights = HashMap::new();
        for (entity_id, entity) in chunk.get_entities() {
            if !entity.can_see(id, entity.coords, self) {
                continue;
            }

            let distance = entity.coords.distance_from(&entity.coords);
            let distance_weight = if distance < 1.0 { 0.0 } else { 1.0 / distance };

            let opinion_weight = self.find_opinion(&entity)?.get_weight();
            let entity_weights = vec![distance_weight, opinion_weight];

            weights.insert(entity_id, product(&entity_weights));
        }
        let values: Vec<&f64> = weights.values().collect();
        insert_noise(&mut values, 0.1, rng);
        let max_weight_id = None;
        let max_weight = -1.0;
        for (id, weight) in weights {
            if weight > max_weight {
                max_weight = weight;
                max_weight_id = Some(*id);
            }
        }

        if max_weight < 0.0005 {
            self.focus = self.generate_solo_focus();
        } else {
            self.focus = self.generate_cooperative_focus(
                max_weight_id,
                chunk.get_entity_from_id(max_weight_id.unwrap()),
            );
        }
    }
}

impl Opinion {
    pub fn get_weight(&self) -> f32 {
        match self {
            Self::Love => 0.2,
            Self::Friendly => 0.1,
            Self::Neutral => 0.03,
            Self::Enemy => 0.7,
            Self::ArchNemesis => 1.0,
        }
    }
}
