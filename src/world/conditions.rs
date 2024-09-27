use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum WorldCondition {
    DiscoveredTimeIsles,
}
