use std::collections::HashMap;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    constants::{MAX_COORDS, MIN_COORDS},
    rng::TickBasedRng,
    types::{Coords3D, Direction3D, FormattedString, GenericStyle, Message},
};

use super::{
    action::{Action, SoloAction},
    entity::{Entity, Focus},
    log::LogStyle,
};

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Chunk {
    entities: HashMap<u32, Entity>,
}

impl Chunk {
    pub fn from_seed(coords: Coords3D, seed: u32, next_entity_id: &mut u32) -> Result<Self> {
        Ok(Default::default())
    }

    pub fn pass_tick(&mut self, rng: &mut TickBasedRng, tick: u32) -> Result<Vec<Message>> {
        let mut messages = vec![];
        let mut ids = vec![];
        for (id, _) in &self.entities {
            ids.push(*id);
        }
        for id in &ids {
            if let Some(entity) = self.get_entity_from_id(*id) {
                if entity.has_agency {
                    if let Some(message) = self.npc_turn(*id, rng, tick)? {
                        messages.push(message);
                    }
                }
            }
        }
        Ok(messages)
    }

    pub fn move_entity(&mut self, id: u32, direction: &Direction3D) -> Result<()> {
        let mover = self
            .get_entity_from_id(id)
            .unwrap_or(Err(anyhow!("can't find entity from id {id} to move"))?);
        let mut coords_try = mover.coords.clone();
        coords_try.move_in_direction(direction);
        if self.is_oob(&coords_try) {
            return Ok(());
        }

        let movee = if let Some(movee) = self.get_entity_from_coords(&coords_try) {
            movee
        } else {
            self.get_entity_mut_from_id(id).unwrap().coords = coords_try;
            return Ok(());
        };

        if movee.can_move && mover.can_move {
            self.get_entity_mut_from_coords(&coords_try).unwrap().coords = mover.coords.clone();
            self.get_entity_mut_from_id(id).unwrap().coords = coords_try;
        }

        Ok(())
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
                return Some(entity.1);
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

    pub fn inspect_entity(&self, id: u32) -> Result<FormattedString<LogStyle>> {
        Ok(self
            .get_entity_from_id(id)
            .unwrap_or(Err(anyhow!(
                "cannot find entity from id {id} for inspection"
            ))?)
            .description
            .clone())
    }

    fn is_oob(&self, coords: &Coords3D) -> bool {
        if (coords.0 > MAX_COORDS.0) || (coords.0 < MIN_COORDS.0) {
            return true;
        }
        if (coords.1 > MAX_COORDS.1) || (coords.1 < MIN_COORDS.1) {
            return true;
        }
        if (coords.2 > MAX_COORDS.2) || (coords.2 < MIN_COORDS.2) {
            return true;
        }

        false
    }

    fn get_entity_mut_from_coords(&mut self, coords: &Coords3D) -> Option<&mut Entity> {
        for entity in &mut self.entities {
            if entity.1.coords == *coords {
                return Some(entity.1);
            }
        }
        None
    }

    fn npc_turn(&mut self, id: u32, rng: &mut TickBasedRng, tick: u32) -> Result<Option<Message>> {
        let entity = self.get_entity_from_id(id).unwrap();

        let focus = if entity.focus.is_none() {
            Some(entity.find_new_focus(id, self, rng, tick))
        } else {
            None
        };

        let entity = self
            .get_entity_mut_from_id(id)
            .ok_or(anyhow!("entity could not be found from id: {id}",))?;

        if focus.is_some() {
            entity.focus = focus;
        }

        let focus = entity.focus.clone().unwrap();
        if let Action::Solo(_) = focus.action {
            self.entity_take_solo_action(id, rng, &focus, tick)
        } else {
            self.entity_interaction(id, rng, &focus, tick)
        }
    }

    fn entity_interaction(
        &mut self,
        id: u32,
        rng: &mut TickBasedRng,
        focus: &Focus,
        tick: u32,
    ) -> Result<Option<Message>> {
        let actor = self
            .entities
            .get(&id)
            .unwrap_or(Err(anyhow!("entity {id} not found for interaction"))?);
        match &focus.action {
            Action::Flee(actee_id) => {
                let actee = self
                    .entities
                    .get(&actee_id)
                    .unwrap_or(Err(anyhow!("entity {actee_id} not found for interaction"))?);
                self.flee(id, &actee.coords, rng)
            }
            Action::Talk(actee_id) => {
                let actee = self
                    .entities
                    .get(&actee_id)
                    .unwrap_or(Err(anyhow!("entity {actee_id} not found for interaction"))?);
                actor.talk_to(actee)
            }
            Action::Fight(actee_id) => {
                let actee = self
                    .entities
                    .get(&actee_id)
                    .unwrap_or(Err(anyhow!("entity {actee_id} not found for interaction"))?);
                actor.fight(actee)    
            }
            Action::Approach(actee_id) => {
                let actee = self
                    .entities
                    .get(&actee_id)
                    .unwrap_or(Err(anyhow!("entity {actee_id} not found for interaction"))?);
                self.approach(id, &actee.coords, rng)
            }
            Action::Solo(_) => Err(anyhow!(
                "entity tried to take a solo action in a cooperative action context"
            )),
        }
    }

    fn entity_take_solo_action(
        &mut self,
        id: u32,
        rng: &mut TickBasedRng,
        focus: &Focus,
        tick: u32,
    ) -> Result<Option<Message>> {
        if let Action::Solo(action) = &focus.action {
            match action {
                SoloAction::Wander => {
                    let val = rng.rand() % 6;
                    let direction = match val {
                        0 => Direction3D::North,
                        1 => Direction3D::East,
                        2 => Direction3D::West,
                        3 => Direction3D::South,
                        4 => Direction3D::Up,
                        5 => Direction3D::Down,
                        _ => unreachable!("{val} can't exceed 5"),
                    };
                    self.move_entity(id, &direction)?;
                }
            }
        } else {
            return Err(anyhow!(
                "entity tried to take a cooperative action in a solo action context"
            ));
        }

        Ok(None)
    }

    pub fn flee(
        &mut self,
        id: u32,
        coords: &Coords3D,
        rng: &mut TickBasedRng,
    ) -> Result<Option<Message>> {
        let entity = self
            .get_entity_from_id(id)
            .unwrap_or(Err(anyhow!("entity {id} not found"))?);
        let mut direction = entity.coords.direction_to(coords);
        direction.invert();

        let possible_directions = match direction {
            Direction3D::North => vec![
                Direction3D::North,
                Direction3D::Northeast,
                Direction3D::Northwest,
            ],
            Direction3D::Northeast => vec![
                Direction3D::Northeast,
                Direction3D::North,
                Direction3D::East,
            ],
            Direction3D::East => vec![
                Direction3D::East,
                Direction3D::Northeast,
                Direction3D::Southeast,
            ],
            Direction3D::Southeast => vec![
                Direction3D::Southeast,
                Direction3D::East,
                Direction3D::South,
            ],
            Direction3D::South => vec![
                Direction3D::South,
                Direction3D::Southeast,
                Direction3D::Southwest,
            ],
            Direction3D::Southwest => vec![
                Direction3D::Southwest,
                Direction3D::South,
                Direction3D::West,
            ],
            Direction3D::West => vec![
                Direction3D::West,
                Direction3D::Southwest,
                Direction3D::Northwest,
            ],
            Direction3D::Northwest => vec![
                Direction3D::Northwest,
                Direction3D::West,
                Direction3D::North,
            ],
            Direction3D::Up => vec![Direction3D::Up],
            Direction3D::Down => vec![Direction3D::Down],
        };

        let index = rng.rand() as usize % possible_directions.len();
        let direction = possible_directions[index];

        self.move_entity(id, &direction);

        Ok(None)
    }
    
    pub fn approach(
        &mut self,
        id: u32,
        coords: &Coords3D,
        rng: &mut TickBasedRng,
    ) -> Result<Option<Message>> {
        let entity = self
            .get_entity_from_id(id)
            .unwrap_or(Err(anyhow!("entity {id} not found"))?);
        let direction = entity.coords.direction_to(coords);

        let possible_directions = match direction {
            Direction3D::North => vec![
                Direction3D::North,
                Direction3D::Northeast,
                Direction3D::Northwest,
            ],
            Direction3D::Northeast => vec![
                Direction3D::Northeast,
                Direction3D::North,
                Direction3D::East,
            ],
            Direction3D::East => vec![
                Direction3D::East,
                Direction3D::Northeast,
                Direction3D::Southeast,
            ],
            Direction3D::Southeast => vec![
                Direction3D::Southeast,
                Direction3D::East,
                Direction3D::South,
            ],
            Direction3D::South => vec![
                Direction3D::South,
                Direction3D::Southeast,
                Direction3D::Southwest,
            ],
            Direction3D::Southwest => vec![
                Direction3D::Southwest,
                Direction3D::South,
                Direction3D::West,
            ],
            Direction3D::West => vec![
                Direction3D::West,
                Direction3D::Southwest,
                Direction3D::Northwest,
            ],
            Direction3D::Northwest => vec![
                Direction3D::Northwest,
                Direction3D::West,
                Direction3D::North,
            ],
            Direction3D::Up => vec![Direction3D::Up],
            Direction3D::Down => vec![Direction3D::Down],
        };

        let index = rng.rand() as usize % possible_directions.len();
        let direction = possible_directions[index];

        self.move_entity(id, &direction);

        Ok(None)
    }
}
