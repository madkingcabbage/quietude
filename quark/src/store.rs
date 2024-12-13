use std::{fs::DirBuilder, path::{Path, PathBuf}};

use anyhow::{anyhow, Result};
use log::info;
use quietude::{constants::{SAVE_EXTENSION, WORLD_DIR_NAME, WORLD_FILENAME}, store::{load, save}, types::Coords4D, world::chunk::Chunk};

use crate::app::App;

pub fn save_project(app: &App) -> Result<()> {
    let project_dir = app.project_dir.as_ref().ok_or(anyhow!("tried to load a chunk without a project directory"))?;
    if !guarantee_project_structure(Path::new(&project_dir))? {
        return Err(anyhow!("project structure was invalid"));
    }

    let parent: PathBuf = [project_dir, WORLD_DIR_NAME].iter().collect();

    let mut world_path = parent.clone();
    world_path.push(Path::new(&format!("{WORLD_FILENAME}{SAVE_EXTENSION}")));

    let mut chunk_path = parent;
    let default_coords = Coords4D(0, 0, 0, 0);
    chunk_path.push(Path::new(&format!("{default_coords}{SAVE_EXTENSION}")));

    save(&world_path, &app.next_valid_entity_id)?;
    save(&chunk_path, &app.world.active_chunk)?;

    Ok(())
}

pub fn load_project(path: &Path) -> Result<(u32, Chunk)> {
    if !guarantee_project_structure(path)? {
        return Err(anyhow!("project structure was invalid"));
    }

    let mut world_path = path.to_path_buf();
    world_path.push(WORLD_DIR_NAME);
    world_path.push(format!("{WORLD_FILENAME}{SAVE_EXTENSION}"));
    info!("Loading world from {}", path.to_string_lossy());
    Ok((
        load(&world_path)?,
        load_chunk(&path, Coords4D(0, 0, 0, 0))?,
    ))
}

pub fn load_chunk(path: &Path, chunk_coords: Coords4D) -> Result<Chunk> {
    if !guarantee_project_structure(&path)? {
        return Err(anyhow!("project structure was invalid"));
    }

    let mut path = path.to_path_buf();
    path.push(WORLD_DIR_NAME);
    path.push(format!("{}{SAVE_EXTENSION}", chunk_coords));
    info!("Loading chunk from {}", path.as_path().to_string_lossy());
    Ok(
        load(path.as_path())?
    )
}

/// Guarantees that all of the necessary directories exist to save a project.
///
/// Returns `false` if new directories had to be created.
pub fn guarantee_project_structure(path: &Path) -> Result<bool> {
    let mut path = path.to_path_buf();
    path.push(WORLD_DIR_NAME);
    if !Path::new(&path).exists() {
        DirBuilder::new()
            .recursive(true)
            .create(&path)?;
        info!("Created project directories in {}", path.to_string_lossy());
        Ok(false)
    } else {
        Ok(true)
    }
}
