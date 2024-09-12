use serde::{Deserialize, Serialize};

use crate::types::Coords3D;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Action {
    Fight(u32),
    Flee(u32),
    Talk(u32),
    Approach(u32),
    Solo(SoloAction),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum SoloAction {
    Wander,
}
