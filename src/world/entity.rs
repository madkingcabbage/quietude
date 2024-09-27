use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    rng::TickBasedRng,
    types::{Coords3D, Direction3D, FormattedString, GenericStyle, LineSegment3D, Message},
    utils::{avg, insert_noise, product},
};

use super::{
    action::{Action, SoloAction},
    chunk::Chunk,
    item::{Item, ItemType},
    log::LogStyle, traits::VisibilityModifier,
};

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
    pub name: Vec<FormattedString<LogStyle>>,
    pub description: FormattedString<LogStyle>,
    pub can_move: bool,
    pub has_agency: bool,
    pub allegiance: Option<String>,
    pub focus: Option<Focus>,
    pub opacity: Opacity,
    pub size: Size,
    pub coords: Coords3D,
    pub inventory: Vec<Item>,
    inventory_spaces: usize,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Focus {
    pub action: Action,
    pub tick_born: u32,
}

#[derive(PartialEq)]
pub enum Opinion {
    Love,
    Friendly,
    Neutral,
    Enemy,
    ArchNemesis,
    Fear,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Opacity {
    Solid,
    Dense,
    MostlyTransparent,
    Transparent,
}

impl Entity {
    pub fn find_new_focus(&self, id: u32, chunk: &Chunk, rng: &mut TickBasedRng, tick: u32) -> Focus {
        let mut weights = HashMap::new();
        for (entity_id, entity) in chunk.get_entities() {
            if !self.can_see(chunk, entity) {
                continue;
            }

            let distance = entity.coords.distance_from(&entity.coords);
            let distance_weight = if distance < 1.0 { 0.0 } else { 1.0 / distance };

            let opinion_weight = self.find_opinion(&entity).get_weight();
            let entity_weights = vec![distance_weight, opinion_weight];

            weights.insert(entity_id, product(&entity_weights));
        }
        let mut values: Vec<&f64> = weights.values().collect();
        insert_noise(&mut values, 0.1, rng);
        let mut max_weight_id = None;
        let mut max_weight = -1.0;
        for (id, weight) in weights {
            if weight > max_weight {
                max_weight = weight;
                max_weight_id = Some(*id);
            }
        }

        if max_weight < 0.0005 {
            self.generate_solo_focus(tick)
        } else {
            self.generate_cooperative_focus(
                max_weight_id.unwrap(),
                chunk.get_entity_from_id(max_weight_id.unwrap()).unwrap(),
                tick,
            )
        }
    }

    pub fn has_item(&self, item: ItemType) -> bool {
        for Item { item_type, .. } in &self.inventory {
            if *item_type == item {
                return true;
            }
        }

        false
    }

    pub fn has_specific_item(&self, item_id: u32) -> bool {
        for Item { id, .. } in &self.inventory {
            if *id == item_id {
                return true;
            }
        }

        false
    }

    pub fn give_item(&mut self, item: ItemType) -> Result<()> {
        if self.inventory.len() >= self.inventory_spaces {
            return Err(anyhow!("item {item} cannot be added to full inventory"));
        }
        self.inventory.push(Item::from_item_type(item)?);
        Ok(())
    }

    pub fn give_specific_item(&mut self, id: u32) -> Result<()> {
        if self.inventory.len() >= self.inventory_spaces {
            return Err(anyhow!(
                "item from id {id} cannot be added to full inventory"
            ));
        }
        self.inventory.push(Item::from_id(id)?);
        Ok(())
    }

    pub fn take_specific_item(&mut self, id: u32) -> Result<()> {
        for (i, Item { id: item_id, .. }) in self.inventory.iter().enumerate() {
            if id == *item_id {
                self.inventory.remove(i);
                return Ok(());
            }
        }
        Err(anyhow!("failed to find item with id {id} in inventory"))
    }

    pub fn can_see(&self, chunk: &Chunk, entity: &Entity) -> bool {
        let ray = LineSegment3D {
            start: self.coords,
            end: entity.coords,
        };

        let intersections = ray.intersects();
        let mut visibility = 1.0;
        for coords in &intersections {
            if let Some(entity) = chunk.get_entity_from_coords(coords) {
                visibility *= entity.visibility_reduction_factor();
            }
        }

        if visibility > 0.5 {
            true
        } else {
            false
        }
    }

    pub fn generate_solo_focus(&self, tick: u32) -> Focus {
        Focus {
            action: Action::Solo(SoloAction::Wander),
            tick_born: tick,
        }
    }

    pub fn generate_cooperative_focus(&self, id: u32, entity: &Entity, tick: u32) -> Focus {
        let distance = self.coords.distance_from(&entity.coords);
        let opinion = self.find_opinion(entity);
        let action = if opinion == Opinion::Fear {
            Action::Flee(id)
        } else if distance > 2.0 {
            Action::Approach(id)
        } else if (opinion == Opinion::Love)
            || (opinion == Opinion::Friendly)
            || (opinion == Opinion::Neutral)
        {
            Action::Talk(id)
        } else if (opinion == Opinion::Enemy) || (opinion == Opinion::ArchNemesis) {
            Action::Fight(id)
        } else {
            Action::Talk(id)
        };

        Focus {
            action,
            tick_born: tick,
        }
    }

    fn find_opinion(&self, entity: &Entity) -> Opinion {
        Opinion::Neutral
    }

}

impl VisibilityModifier for Opacity {
    fn visibility_reduction_factor(&self) -> f64 {
        match self {
            Opacity::Transparent => 0.0,
            Opacity::MostlyTransparent => 0.1,
            Opacity::Dense => 0.3,
            Opacity::Solid => 1.0,
        }
    }
}

impl VisibilityModifier for Size {
    fn visibility_reduction_factor(&self) -> f64 {
        match self {
            Size::Small => 0.1,
            Size::Medium => 0.3,
            Size::Large => 0.7,
        }
    }
}

impl VisibilityModifier for Entity {
    fn visibility_reduction_factor(&self) -> f64 {
        self.opacity.visibility_reduction_factor() * self.size.visibility_reduction_factor()
    }
}

impl Opinion {
    pub fn get_weight(&self) -> f64 {
        match self {
            Self::Love => 0.2,
            Self::Friendly => 0.1,
            Self::Neutral => 0.03,
            Self::Enemy => 0.6,
            Self::ArchNemesis => 0.8,
            Self::Fear => 1.0,
        }
    }
}
