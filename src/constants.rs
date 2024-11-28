use std::str;

use crate::types::Coords3D;

pub const WORLD_FILENAME: &str = "world";
pub const WORLD_DIR_NAME: &str = "world/";
pub const SAVE_EXTENSION: &str = ".json";

pub const MAX_COORDS: Coords3D = Coords3D(31, 15, 1);
pub const MIN_COORDS: Coords3D = Coords3D(-32, -16, 0);

