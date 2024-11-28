use std::{
    fs::{create_dir_all, DirBuilder, File},
    io::BufWriter,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use include_dir::{include_dir, Dir};
use log::{info, trace};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};

pub static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub fn save_profile<T: Serialize>(path: &str, data: &T) -> Result<()> {
    let file = File::create(store_path(path)?)?;
    assert!(file.metadata()?.is_file());
    let buffer = BufWriter::new(file);
    to_writer(buffer, data)?;
    Ok(())
}

pub fn load_profile<T>(filename: &str) -> Result<T>
where
    for<'a> T: Deserialize<'a>,
{
    let file = File::open(store_path(filename)?)?;
    let data: T = from_reader(file)?;
    Ok(data)
}

pub fn save<T: Serialize>(path: &PathBuf, data: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !Path::new(parent).exists() {
            DirBuilder::new().recursive(true).create(&path)?;
        }
    }
    let file = File::create(path)?;
    assert!(file.metadata()?.is_file());
    let buffer = BufWriter::new(file);
    to_writer(buffer, data)?;
    trace!("Saved data at {}", path.to_str().unwrap());
    Ok(())
}

pub fn load<T>(path: &PathBuf) -> Result<T>
where
    for<'a> T: Deserialize<'a>,
{
    let file = File::open(path)?;
    let data: T = from_reader(file)?;
    Ok(data)
}

fn store_path(filename: &str) -> Result<PathBuf> {
    let dirs = ProjectDirs::from("org", "pythagorea", "q")
        .ok_or(anyhow!("failed to find directory names"))?;

    let data_dirs = dirs.data_dir();
    if !data_dirs.exists() {
        create_dir_all(data_dirs)?;
    }
    let path = data_dirs.join(filename);
    Ok(path)
}

#[cfg(test)]
mod tests {

    use crate::world::world::World;

    use super::*;

    #[test]
    fn test_save_load() {
        let result = save_profile("test.json", &World::default());
        assert!(result.is_ok());

        let world = load_profile("test.json");
        assert!(result.is_ok());

        let world_some: World = world.unwrap();
        assert_eq!(world_some, World::default());
    }
}
