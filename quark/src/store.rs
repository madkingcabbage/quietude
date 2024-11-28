use std::{fs::DirBuilder, path::{Path, PathBuf}};

use anyhow::{anyhow, Result};
use log::info;
use quietude::{constants::{SAVE_EXTENSION, WORLD_DIR_NAME, WORLD_FILENAME}, store::{load, save}, types::Coords4D, world::chunk::Chunk};

use crate::app::App;

pub fn save_project(app: &App) -> Result<()> {
    let project_dir = app.project_dir.as_ref().ok_or(anyhow!("tried to load a chunk without a project directory"))?;
    if !guarantee_project_structure(&project_dir)? {
        return Err(anyhow!("project structure was invalid"));
    }

    let parent = PathBuf::from(format!("{}{WORLD_DIR_NAME}", project_dir));
    
    let mut world_path = parent.clone();
    world_path.push(Path::new(&format!("{WORLD_FILENAME}{SAVE_EXTENSION}")));

    let mut chunk_path = parent;
    let default_coords = Coords4D(0, 0, 0, 0);
    chunk_path.push(Path::new(&format!("{default_coords}{SAVE_EXTENSION}")));

    save(&world_path, &app.next_valid_entity_id)?;
    save(&chunk_path, &app.world.active_chunk)?;

    Ok(())
}

pub fn load_project(path: &str) -> Result<(u32, Chunk)> {
    if !guarantee_project_structure(path)? {
        return Err(anyhow!("project structure was invalid"));
    }

    let load_location = format!("{}{WORLD_DIR_NAME}{WORLD_FILENAME}{SAVE_EXTENSION}", path);
    info!("Loading world from {load_location}");
    Ok((
        load(&PathBuf::from(&load_location))?,
        load_chunk(path, Coords4D(0, 0, 0, 0))?,
    ))
}

pub fn load_chunk(path: &str, chunk_coords: Coords4D) -> Result<Chunk> {
    if !guarantee_project_structure(&path)? {
        return Err(anyhow!("project structure was invalid"));
    }

    let load_location = format!("{path}{WORLD_DIR_NAME}{chunk_coords}{SAVE_EXTENSION}");
    info!("Loading chunk from {load_location}");
    Ok(
        load(&PathBuf::from(&load_location))?
    )
}

/// Guarantees that all of the necessary directories exist to save a project.
///
/// Returns `false` if new directories had to be created.
pub fn guarantee_project_structure(path: &str) -> Result<bool> {
    let parent = format!("{path}{WORLD_DIR_NAME}");
    if !Path::new(&parent).exists() {
        DirBuilder::new()
            .recursive(true)
            .create(&parent)?;
        info!("Created project directories");
        Ok(false)
    } else {
        Ok(true)
    }
}
