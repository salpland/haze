use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::error::Error;

/// Exports the given world to the `minecraftWorlds` directory.
pub fn export(name: &str, worlds: &[String], overwrite: bool) -> Result<(), Error> {
    let from = local_worlds_dir(worlds, name)?;
    let to = mojang_worlds_dir(name)?;

    if to.exists() && !overwrite {
        return Err(Error::CannotOverwriteWorld(name.to_string()));
    }

    copy_dir(&from, &to).map_err(|e| Error::CannotCopyWorld(e.kind(), name.to_string()))?;

    Ok(())
}

/// Imports the given world from the `minecraftWorlds` directory.
pub fn import(name: &str, worlds: &[String]) -> Result<(), Error> {
    let from = mojang_worlds_dir(name)?;
    let to: PathBuf = local_worlds_dir(worlds, name)?;

    copy_dir(&from, &to).map_err(|e| Error::CannotCopyWorld(e.kind(), name.to_string()))?;

    Ok(())
}

/// Returns the list of worlds from the given glob patterns.
pub fn all_worlds(globs: &[String]) -> Result<Vec<String>, Error> {
    let worlds = globs
        .iter()
        .map(|pattern| {
            glob::glob(pattern).map_err(|e| Error::InvalidWorldsGlobPattern(e, pattern.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|p| p.is_dir())
        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
        .collect::<Vec<_>>();

    if worlds.is_empty() {
        return Err(Error::EmptyWorldsProperty);
    }

    Ok(worlds)
}

/// Returns local worlds directory from the given glob patterns.
fn local_worlds_dir(globs: &[String], name: &str) -> Result<PathBuf, Error> {
    let paths = globs
        .iter()
        .map(|pattern| {
            glob::glob(pattern).map_err(|e| Error::InvalidWorldsGlobPattern(e, pattern.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .filter(|p| p.is_dir())
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return Err(Error::EmptyWorldsProperty);
    }

    match paths.iter().find(|p| p.ends_with(name)) {
        Some(path) => Ok(path.clone()),
        None => Err(Error::WorldNotFound(paths, name.to_string())),
    }
}

/// Returns the path to the mojang worlds directory.
fn mojang_worlds_dir(name: &str) -> Result<PathBuf, Error> {
    env::var("LOCALAPPDATA")
        .map(|base| {
            PathBuf::from(&base)
                .join("Packages")
                .join("Microsoft.MinecraftUWP_8wekyb3d8bbwe")
                .join("LocalState")
                .join("games")
                .join("com.mojang")
                .join("minecraftWorlds")
                .join(name)
        })
        .ok()
        .ok_or_else(Error::CannotFindLocalAppData)
}

/// Copies a directory recursively.
fn copy_dir(from: &Path, to: &Path) -> Result<(), io::Error> {
    fs::create_dir_all(to)?;

    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            fs::copy(&entry.path(), &to.join(entry.file_name()))?;
        }
    }

    Ok(())
}
