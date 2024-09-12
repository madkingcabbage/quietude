use std::{fs::{create_dir_all, File}, io::BufWriter, path::PathBuf};

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};

pub static ASSETS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");

pub fn save<T: Serialize>(filename: &str, data: &T) -> Result<()> {
    let file = File::create(store_path(filename)?)?;
    assert!(file.metadata()?.is_file());
    let buffer = BufWriter::new(file);
    to_writer(buffer, data)?;
    Ok(())
}

pub fn load<T>(filename: &str) -> Result<T> 
    where for<'a> T: Deserialize<'a> {
    let file = File::open(store_path(filename)?)?;
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
        let result = save("test.json", &World::default());
        assert!(result.is_ok());

        let world = load("test.json");
        assert!(result.is_ok());

        let world_some: World = world.unwrap();
        assert_eq!(world_some, World::default());
    }
}
