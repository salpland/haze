use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::error::{HazeError, HazeResult};

pub fn test(world_name: &str, local_worlds_dir: &str) -> HazeResult<()> {
    let from: PathBuf = [local_worlds_dir, world_name].iter().collect();
    let to = make_mojang_worlds_dir(world_name).map_err(HazeError::LocalAppData)?;

    copy_dir(from, to).map_err(|e| HazeError::WorldCopy(e, world_name.to_string()))
}

pub fn save(world_name: &str, local_worlds_dir: &str) -> HazeResult<()> {
    let from = make_mojang_worlds_dir(world_name).map_err(HazeError::LocalAppData)?;
    let to: PathBuf = [local_worlds_dir, world_name].iter().collect();

    copy_dir(from, to).map_err(|e| HazeError::WorldCopy(e, world_name.to_string()))
}

fn make_mojang_worlds_dir(name: &str) -> Result<PathBuf, io::Error> {
    let base = env::var("LOCALAPPDATA").unwrap();

    Ok([
        &base,
        "Packages",
        "Microsoft.MinecraftUWP_8wekyb3d8bbwe",
        "LocalState",
        "games",
        "com.mojang",
        "minecraftWorlds",
        name,
    ]
    .iter()
    .collect())
}

fn copy_dir(from: impl AsRef<Path>, to: impl AsRef<Path>) -> Result<(), io::Error> {
    fs::create_dir_all(&to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(entry.path(), to.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
