use std::{collections::HashMap, default, fmt::Display, result, str::FromStr, sync::OnceLock};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::{
    rng::TickBasedRng,
    types::{Coords3D, FormattedString, FormattedText, LineSegment3D},
    ui::traits::{ChoiceAttribute, StringLookup, StringLookupDictionary},
    utils::{insert_noise, product},
};

use super::{
    action::{Action, SoloAction}, allegiance::Allegiance, chunk::Chunk, item::{Item, ItemType}, log::LogStyle, traits::VisibilityModifier
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

impl StringLookup for EntityType {
    fn dictionary() -> &'static StringLookupDictionary<Self> {
        static DICT: OnceLock<StringLookupDictionary<EntityType>> = OnceLock::new();
        DICT.get_or_init(|| {
            let dict = vec![
                (&EntityType::Player, "Player"),
                (&EntityType::Grass, "Grass"),
                (&EntityType::Tree, "Tree"),
                (&EntityType::NpcFriendly, "Friendly NPC"),
                (&EntityType::Void, "Void"),
            ];
            let dict = StringLookupDictionary { dict };
            dict
        })
    }
}

impl ChoiceAttribute for EntityType {}

pub static mut NEXT_ID: u32 = 0;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Clone)]
pub struct Entity {
    pub entity_type: EntityType,
    pub name: FormattedString<LogStyle>,
    pub description: FormattedString<LogStyle>,
    pub is_rooted: Option<bool>,
    pub has_agency: Option<bool>,
    pub allegiance: Option<String>,
    pub opacity: Option<Opacity>,
    pub size: Option<Size>,
    pub coords: Coords3D,
    pub inventory: Vec<Item>,
    pub focus: Option<Focus>,
    pub inventory_spaces: usize,
}

const IS_ROOTED_DEFAULT: bool = true;
const HAS_AGENCY_DEFAULT: bool = false;

#[derive(Clone)]
pub enum EntityAttribute {
    Text(EntityAttributeText),
    Choice(EntityAttributeChoice),
}

#[derive(Clone, Copy, Default)]
pub enum EntityAttributeText {
    #[default]
    Name,
    Description,
}

#[derive(Clone, Default)]
pub enum EntityAttributeChoice {
    #[default]
    Type,
    IsRooted,
    HasAgency,
    Allegiance,
    Opacity,
    Size,
}

const ATTRIBUTE_COUNT: usize = 8;
const ATTRIBUTE_ORDER: [EntityAttribute; ATTRIBUTE_COUNT] = [
    EntityAttribute::Choice(EntityAttributeChoice::Type),
    EntityAttribute::Text(EntityAttributeText::Name),
    EntityAttribute::Text(EntityAttributeText::Description),
    EntityAttribute::Choice(EntityAttributeChoice::IsRooted),
    EntityAttribute::Choice(EntityAttributeChoice::HasAgency),
    EntityAttribute::Choice(EntityAttributeChoice::Allegiance),
    EntityAttribute::Choice(EntityAttributeChoice::Opacity),
    EntityAttribute::Choice(EntityAttributeChoice::Size),
];

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

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl StringLookup for Size {
    fn dictionary() -> &'static StringLookupDictionary<Self> {
        static DICT: OnceLock<StringLookupDictionary<Size>> = OnceLock::new();
        DICT.get_or_init(|| {
            let dict = vec![
                (&Size::Small, "Small"),
                (&Size::Medium, "Medium"),
                (&Size::Large, "Large"),
            ];
            let dict = StringLookupDictionary {
                dict
            };
            dict
        })
    }
}

impl ChoiceAttribute for Size {}

#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Opacity {
    #[default]
    Solid,
    Dense,
    MostlyTransparent,
    Transparent,
}

impl StringLookup for Opacity {
    fn dictionary() -> &'static StringLookupDictionary<Self> {
        static DICT: OnceLock<StringLookupDictionary<Opacity>> = OnceLock::new();
        DICT.get_or_init(|| {
            let dict = vec![
                (&Opacity::Solid, "Solid"), 
                (&Opacity::Dense, "Dense"),
                (&Opacity::MostlyTransparent, "Mostly Transparent"),
                (&Opacity::Transparent, "Transparent"),
            ];
            let dict = StringLookupDictionary {
                dict
            };
            dict
        })
    }
}

impl ChoiceAttribute for Opacity {}

impl Entity {
    pub fn new(coords: &Coords3D) -> Self {
        let mut entity = Entity::default();
        entity.coords = coords.clone();
        entity
    }

    pub fn find_new_focus(
        &self,
        id: u32,
        chunk: &Chunk,
        rng: &mut TickBasedRng,
        tick: u32,
    ) -> Focus {
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

    pub fn get_attribute_value(&self, attr: &EntityAttribute) -> FormattedString<LogStyle> {
        match attr {
            EntityAttribute::Text(attr_text) => match attr_text {
                EntityAttributeText::Name => self.name.clone(),
                EntityAttributeText::Description => self.description.clone(),
            },
            EntityAttribute::Choice(attr_choice) => match attr_choice {
                EntityAttributeChoice::Type => FormattedString::from(
                    &None,
                    FormattedText::new(&format!("{}", self.entity_type), LogStyle::Value),
                ),
                EntityAttributeChoice::IsRooted => FormattedString::from(
                    &None,
                    FormattedText::new(&format!("{}", self.is_rooted()), LogStyle::Value),
                ),
                EntityAttributeChoice::HasAgency => FormattedString::from(
                    &None,
                    FormattedText::new(&format!("{}", self.has_agency()), LogStyle::Value),
                ),
                EntityAttributeChoice::Allegiance => FormattedString::from(
                    &None,
                    FormattedText::new(&format!("{}", self.allegiance()), LogStyle::Value),
                ),
                EntityAttributeChoice::Opacity => FormattedString::from(
                    &None,
                    FormattedText::new(&format!("{}", self.opacity()), LogStyle::Value),
                ),
                EntityAttributeChoice::Size => FormattedString::from(
                    &None,
                    FormattedText::new(&format!("{}", self.size()), LogStyle::Value),
                ),
            },
        }
    }

    pub fn attribute_count(&self) -> usize {
        let mut count = 0;
        for attr in EntityAttribute::attribute_order() {
            if self.has_attribute(attr) {
                count += 1;
            }
        }
        count
    }

    pub fn add_attribute(&mut self, attr: &EntityAttribute) -> Result<()> {
        match attr {
            EntityAttribute::Text(attr) => {
                match attr {
                    EntityAttributeText::Name => Err(anyhow!("{attr} is already a required field")),
                    EntityAttributeText::Description => Err(anyhow!("{attr} is already a required field")),
                }
            }
            EntityAttribute::Choice(attr) => {
                match attr {
                    EntityAttributeChoice::Type => Err(anyhow!("{attr} is already a required field")),
                    EntityAttributeChoice::IsRooted => {
                        if self.is_rooted.is_some() {
                            Err(anyhow!("{attr} is already present"))
                        } else {
                            self.is_rooted = Some(Default::default());
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::HasAgency => {
                        if self.is_rooted.is_some() {
                            Err(anyhow!("{attr} is already present"))
                        } else {
                            self.has_agency = Some(Default::default());
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::Allegiance => {
                        if self.allegiance.is_some() {
                            Err(anyhow!("{attr} is already present"))
                        } else {
                            self.allegiance = Some(Default::default());
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::Opacity => {
                        if self.opacity.is_some() {
                            Err(anyhow!("{attr} is already present"))
                        } else {
                            self.opacity = Some(Default::default());
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::Size => {
                        if self.size.is_some() {
                            Err(anyhow!("{attr} is already present"))
                        } else {
                            self.size = Some(Default::default());
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    pub fn remove_attribute(&mut self, attr: &EntityAttribute) -> Result<()> {
        match attr {
            EntityAttribute::Text(attr) => {
                match attr {
                    EntityAttributeText::Name => Err(anyhow!("{attr} is a required field")),
                    EntityAttributeText::Description => Err(anyhow!("{attr} is a required field")),
                }
            }
            EntityAttribute::Choice(attr) => {
                match attr {
                    EntityAttributeChoice::Type => Err(anyhow!("{attr} is a required field")),
                    EntityAttributeChoice::IsRooted => {
                        if self.is_rooted.is_none() {
                            Err(anyhow!("{attr} does not exist in entity"))
                        } else {
                            self.is_rooted = None;
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::HasAgency => {
                        if self.has_agency.is_none() {
                            Err(anyhow!("{attr} does not exist in entity"))
                        } else {
                            self.has_agency = None;
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::Allegiance => {
                        if self.allegiance.is_none() {
                            Err(anyhow!("{attr} does not exist in entity"))
                        } else {
                            self.allegiance = None;
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::Opacity => {
                        if self.opacity.is_none() {
                            Err(anyhow!("{attr} does not exist in entity"))
                        } else {
                            self.opacity = None;
                            Ok(())
                        }
                    }
                    EntityAttributeChoice::Size => {
                        if self.size.is_none() {
                            Err(anyhow!("{attr} does not exist in entity"))
                        } else {
                            self.size = None;
                            Ok(())
                        }
                    }
                }
            }
        }
    }

    pub fn has_attribute(&self, attr: &EntityAttribute) -> bool {
        match attr {
            EntityAttribute::Text(attr_text) => match attr_text {
                EntityAttributeText::Name => true,
                EntityAttributeText::Description => true,
            },
            EntityAttribute::Choice(attr_choice) => match attr_choice {
                EntityAttributeChoice::Type => true,
                EntityAttributeChoice::IsRooted => {
                    if let Some(_) = self.is_rooted {
                        true
                    } else {
                        false
                    }
                }
                EntityAttributeChoice::HasAgency => {
                    if let Some(_) = self.has_agency {
                        true
                    } else {
                        false
                    }
                }
                EntityAttributeChoice::Allegiance => {
                    if let Some(_) = self.allegiance {
                        true
                    } else {
                        false
                    }
                }
                EntityAttributeChoice::Opacity => {
                    if let Some(_) = self.opacity {
                        true
                    } else {
                        false
                    }
                }
                EntityAttributeChoice::Size => {
                    if let Some(_) = self.size {
                        true
                    } else {
                        false
                    }
                }
            },
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

    pub fn opacity(&self) -> Opacity {
        self.opacity.as_ref().unwrap_or(&Opacity::default()).clone()
    }

    pub fn size(&self) -> Size {
        self.size.as_ref().unwrap_or(&Size::default()).clone()
    }

    pub fn allegiance(&self) -> String {
        self.allegiance
            .as_ref()
            .unwrap_or(&String::from("None"))
            .clone()
    }

    pub fn has_agency(&self) -> bool {
        self.has_agency.unwrap_or(HAS_AGENCY_DEFAULT)
    }

    pub fn is_rooted(&self) -> bool {
        self.is_rooted.unwrap_or(IS_ROOTED_DEFAULT)
    }
}

impl EntityAttribute {
    pub fn attribute_order() -> &'static [EntityAttribute; ATTRIBUTE_COUNT] {
        &ATTRIBUTE_ORDER
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
        self.opacity().visibility_reduction_factor() * self.size().visibility_reduction_factor()
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

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Opacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for EntityAttribute {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "Name" => Ok(EntityAttribute::Text(EntityAttributeText::Name)),
            "Description" => Ok(EntityAttribute::Text(EntityAttributeText::Description)),
            "Type" => Ok(EntityAttribute::Choice(EntityAttributeChoice::Type)),
            "Is rooted" => Ok(EntityAttribute::Choice(EntityAttributeChoice::IsRooted)),
            "Has agency" => Ok(EntityAttribute::Choice(EntityAttributeChoice::HasAgency)),
            "Allegiance" => Ok(EntityAttribute::Choice(EntityAttributeChoice::Allegiance)),
            "Opacity" => Ok(EntityAttribute::Choice(EntityAttributeChoice::Opacity)),
            "Size" => Ok(EntityAttribute::Choice(EntityAttributeChoice::Size)),
            _ => Err(anyhow!("could not parse {s} as entity attribute")),
        }
    }
}

impl Display for EntityAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityAttribute::Text(attr_text) => write!(f, "{attr_text}"),
            EntityAttribute::Choice(attr_choice) => write!(f, "{attr_choice}"),
        }
    }
}

impl Display for EntityAttributeText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityAttributeText::Name => write!(f, "Name"),
            EntityAttributeText::Description => write!(f, "Description"),
        }
    }
}

impl Display for EntityAttributeChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntityAttributeChoice::Type => write!(f, "Type"),
            EntityAttributeChoice::IsRooted => write!(f, "Is rooted"),
            EntityAttributeChoice::HasAgency => write!(f, "Has agency"),
            EntityAttributeChoice::Allegiance => write!(f, "Allegiance"),
            EntityAttributeChoice::Opacity => write!(f, "Opacity"),
            EntityAttributeChoice::Size => write!(f, "Size"),
        }
    }
}

impl EntityAttributeChoice {
    pub fn choices(&self) -> Vec<&'static str> {
        match self {
            EntityAttributeChoice::Type => EntityType::choices(),
            EntityAttributeChoice::IsRooted => bool::choices(),
            EntityAttributeChoice::HasAgency => bool::choices(),
            EntityAttributeChoice::Allegiance => Allegiance::choices(),
            EntityAttributeChoice::Opacity => Opacity::choices(),
            EntityAttributeChoice::Size => Size::choices(),
        }
    }
}
