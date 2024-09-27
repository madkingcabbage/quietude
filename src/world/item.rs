use std::{
    fmt::Display,
    sync::{
        atomic::{AtomicU32, AtomicUsize, Ordering},
        Mutex, OnceLock,
    },
};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::store::ASSETS_DIR;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Item {
    pub id: u32,
    pub item_type: ItemType,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct ItemData {
    item_type: ItemType,
    name: String,
    description: String,
    damage: u32,
    range: u8,
    unique_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ItemType {
    Book(BookType),
    Weapon(WeaponType),
    Armour(ArmourType),
    Torch,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum BookType {
    ILoveYou,
    Babel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum WeaponType {
    ShortSword,
    BastardSword,
    Zweihander,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ArmourType {
    Cloak,
    Hauberk,
    Knight,
}

impl ItemData {
    fn items_data() -> &'static Vec<ItemData> {
        static DATA: OnceLock<Vec<ItemData>> = OnceLock::new();
        DATA.get_or_init(|| {
            let file = ASSETS_DIR
                .get_file("data/items_data.json")
                .expect("Could not find items_data.json");

            let data = file
                .contents_utf8()
                .expect("Could not read items_data.json");

            serde_json::from_str(&data).unwrap_or_else(|e| {
                panic!("Could not parse items_data.json: {}", e);
            })
        })
    }
}

static ITEM_NEXT_ID: AtomicU32 = AtomicU32::new(RESERVED_ID_COUNT);
static RESERVED_ID_COUNT: u32 = 1024;
impl Item {
    pub fn new(item_type: ItemType, name: &str) -> Result<Self> {
        let id = ITEM_NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Ok(Item {
            id,
            item_type,
            name: String::from(name),
        })
    }

    pub fn new_with_id(item_type: ItemType, name: &str, id: u32) -> Result<Self> {
        if id >= RESERVED_ID_COUNT {
            return Err(anyhow!("requested ID exceeds maximum specific ID"));
        }
        Ok(Item {
            id,
            item_type,
            name: String::from(name),
        })
    }

    pub fn from_item_type(item_type: ItemType) -> Result<Self> {
        for ItemData {
            item_type,
            unique_id,
            name,
            ..
        } in ItemData::items_data()
        {
            if item_type == item_type {
                if unique_id.is_some() {
                    return Ok(Item::new_with_id(
                        item_type.clone(),
                        name,
                        unique_id.unwrap(),
                    )?);
                }
                return Ok(Item::new(item_type.clone(), &name)?);
            }
        }

        Err(anyhow!("could not find item from item type {item_type}"))
    }

    pub fn from_id(id: u32) -> Result<Self> {
        for ItemData {
            item_type,
            unique_id,
            name,
            ..
        } in ItemData::items_data()
        {
            if unique_id.is_some() && unique_id.unwrap() == id {
                return Ok(Item::new_with_id(
                    item_type.clone(),
                    name,
                    unique_id.unwrap(),
                )?);
            }
        }
        
        Err(anyhow!("could not find item from item id {id}"))
    }
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ItemType::Book(book) => format!("book: {book}"),
            ItemType::Weapon(weapon) => format!("weapon: {weapon}"),
            ItemType::Armour(armour) => format!("armour: {armour}"),
            ItemType::Torch => format!("torch"),
        };
        write!(f, "{}", s)
    }
}

impl Display for BookType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for ArmourType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
