use std::path::PathBuf;

use anyhow::{anyhow, Result};
use ratatui::style::Style;
use serde::{Deserialize, Serialize};

use crate::{
    store::{load_profile, save},
    types::{Color, FormattedString},
};

use super::{conditions::WorldCondition, item::ItemType, world::World};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DialogueTree {
    nodes: Vec<DialogueNode>,
    active_node_name: String,
    pub speaker_name: String,
    pub speaker_id: u32,
    pub interlocutor_id: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DialogueNode {
    name: String,
    speaker_dialogue: FormattedString<DialogueStyle>,
    choices: Vec<DialogueChoice>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DialogueChoice {
    text: FormattedString<DialogueStyle>,
    preconditions: Vec<DialoguePrecondition>,
    outcomes: Vec<DialogueOutcome>,
    destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DialoguePrecondition {
    InterlocutorHasItem(ItemType),
    InterlocutorHasSpecificItem(u32),
    WorldConditionIsActive(WorldCondition),
    WorldConditionIsInactive(WorldCondition),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum DialogueOutcome {
    GiveInterlocutorItem(ItemType),
    GiveInterlocutorSpecificItem(u32),
    TakeInterlocutorSpecificItem(u32),
    AddWorldCondition(WorldCondition),
    RemoveWorldCondition(WorldCondition),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum DialogueStyle {
    #[default]
    Default,
    Emphasis(Color),
}

impl DialogueTree {
    pub fn new(speaker_name: &str) -> DialogueTree {
        DialogueTree {
            nodes: vec![],
            active_node_name: String::from("start"),
            speaker_name: speaker_name.to_string(),
            speaker_id: 0,
            interlocutor_id: 0,
        }
    }

    pub fn save(&self, path: PathBuf) -> Result<()> {
        save(&path, &(&self.speaker_name, &self.nodes))
    }

    pub fn from_entity_name(speaker_id: u32, interlocutor_id: u32) -> Result<Self> {
        let (speaker_name, nodes) = load_profile(&format!("{speaker_id}.json"))?;
        Ok(DialogueTree {
            speaker_name,
            nodes,
            active_node_name: String::from("start"),
            speaker_id,
            interlocutor_id,
        })
    }

    pub fn get_active_node(&self) -> Result<&DialogueNode> {
        for node in &self.nodes {
            if node.name == self.active_node_name {
                return Ok(node);
            }
        }

        Err(anyhow!("active node {} not found", self.active_node_name))
    }

    pub fn get_outcomes_and_destination_from_choice(
        &self,
        choice: usize,
        world: &World,
    ) -> Result<(Vec<DialogueOutcome>, String)> {
        Ok(self.get_active_node()?
            .get_outcomes_and_destination_from_choice(choice, world)?)
    }

    pub fn make_choice(&mut self, destination: &str) {
        self.active_node_name = String::from(destination);
    }
}

impl DialogueNode {
    pub fn choices_text(
        &self,
        world: &World,
    ) -> Result<Vec<FormattedString<DialogueStyle>>> {
        let choices = self.choices(world)?;
        let text = choices.iter().map(|choice| choice.text.clone()).collect::<Vec<_>>();
        Ok(text)
    }
    
    pub fn choices_text_unconditional(
        &self,
    ) -> Result<Vec<FormattedString<DialogueStyle>>> {
        let choices = self.choices_unconditional();
        let text = choices.iter().map(|choice| choice.text.clone()).collect::<Vec<_>>();
        Ok(text)
    }

    pub fn choices_count_unconditional(&self) -> usize {
        self.choices.len()
    }

    fn choices(
        &self,
        world: &World,
    ) -> Result<Vec<DialogueChoice>> {
        let mut choices = Vec::new();
        for choice in &self.choices {
            if choice.preconditions_are_met(world)? {
                choices.push(choice.clone());
            }
        }

        Ok(choices)
    }

    fn choices_unconditional(
        &self,
    ) -> &Vec<DialogueChoice> {
        &self.choices
    }

    pub fn speaker_dialogue(&self) -> &FormattedString<DialogueStyle> {
        &self.speaker_dialogue
    }

    fn get_outcomes_and_destination_from_choice(
        &self,
        index: usize,
        world: &World,
    ) -> Result<(Vec<DialogueOutcome>, String)> {
        let choices = self.choices(world)?;
        if index >= choices.len() {
            return Err(anyhow!(
                "could not find choice at index {index} in dialogue node"
            ));
        }
        Ok((choices[index].outcomes.clone(), choices[index].destination.clone()))
    }
}

impl DialogueChoice {
    pub fn preconditions_are_met(
        &self,
        world: &World,
    ) -> Result<bool> {
        for condition in &self.preconditions {
            if !condition.is_met(world)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

}

impl DialoguePrecondition {
    fn is_met(&self, world: &World) -> Result<bool> {
        let interlocutor_id = world.dialogue_tree.as_ref().unwrap().interlocutor_id;
        let speaker_id = world.dialogue_tree.as_ref().unwrap().speaker_id;
        let outcome = match self {
            DialoguePrecondition::InterlocutorHasItem(item) => world
                .active_chunk
                .get_entity_from_id(interlocutor_id)
                .ok_or(anyhow!("interlocutor {} not found", interlocutor_id))?
                .has_item(item.clone()),
            DialoguePrecondition::InterlocutorHasSpecificItem(id) => world
                .active_chunk
                .get_entity_from_id(speaker_id)
                .ok_or(anyhow!("interlocutor {} not found", speaker_id))?
                .has_specific_item(*id),
            DialoguePrecondition::WorldConditionIsActive(condition) => {
                world.has_condition(*condition)
            }
            DialoguePrecondition::WorldConditionIsInactive(condition) => {
                !world.has_condition(*condition)
            }
        };

        Ok(outcome)
    }
}

// TODO: implement!
impl From<DialogueStyle> for Style {
    fn from(value: DialogueStyle) -> Self {
        Style::new()
    }
}
