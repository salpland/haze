use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use colored::Colorize;
use glob::glob;

use crate::error::Error;

/// Exports the given world to the `minecraftWorlds` directory.
pub fn export(name: String, worlds: Vec<String>, overwrite: bool) -> Result<(), Error> {
    let from: PathBuf = local_worlds_dir(worlds, name.clone())?;
    let to = mojang_worlds_dir(&name).map_err(|e| Error::CannotFindLocalAppData(e.kind()))?;

    if to.exists() && !overwrite {
        return Err(Error::CannotOverwriteWorld(name));
    }

    copy_dir(from, to).map_err(|e| Error::CannotCopyWorld(e.kind(), name.clone()))?;

    if overwrite {
        println!(
            "{} world \"{}\" in the \"minecraftWorlds\" directory ({})",
            "Updated".bold().green(),
            name,
            "overwrite".red()
        );
    } else {
        println!(
            "{} world \"{}\" to the \"minecraftWorlds\" directory for testing",
            "Copied".bold().green(),
            name
        );
    }
    Ok(())
}

/// Imports the given world from the `minecraftWorlds` directory.
pub fn import(name: String, worlds: Vec<String>) -> Result<(), Error> {
    let from = mojang_worlds_dir(&name).map_err(|e| Error::CannotFindLocalAppData(e.kind()))?;
    let to: PathBuf = local_worlds_dir(worlds, name.clone())?;

    copy_dir(from, to).map_err(|e| Error::CannotCopyWorld(e.kind(), name.clone()))?;

    println!(
        "{} world \"{}\" to the local worlds directory",
        "Saved".bold().green(),
        name
    );
    Ok(())
}

/// Returns the list of worlds from the given glob patterns.
pub fn all_worlds(globs: Vec<String>) -> Result<Vec<String>, Error> {
    let mut worlds: Vec<String> = Vec::new();
    for pattern in globs.clone() {
        for path in glob(&pattern).map_err(|e| Error::InvalidWorldsGlobPattern(e, pattern))? {
            match path {
                Ok(path) => {
                    if path.is_dir() {
                        if let Some(file_name) = path.file_name() {
                            worlds.push(file_name.to_string_lossy().to_string())
                        }
                    }
                }
                Err(e) => return Err(Error::WorldsDirectoryNotFound(e.error().kind(), globs)),
            }
        }
    }

    if worlds.is_empty() {
        return Err(Error::EmptyWorldsProperty);
    }

    Ok(worlds)
}

/// Returns local worlds directory from the given glob patterns.
fn local_worlds_dir(globs: Vec<String>, name: String) -> Result<PathBuf, Error> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for pattern in globs.clone() {
        for path in glob(&pattern).map_err(|e| Error::InvalidWorldsGlobPattern(e, pattern))? {
            match path {
                Ok(path) => {
                    if path.is_dir() {
                        paths.push(path);
                    }
                }
                Err(e) => return Err(Error::WorldsDirectoryNotFound(e.error().kind(), globs)),
            }
        }
    }

    if paths.is_empty() {
        return Err(Error::EmptyWorldsProperty);
    }
    let Some(path) = paths.iter().find(|p| p.ends_with(&name)) else {
        return Err(Error::WorldNotFound(paths, name));
    };

    Ok(path.clone())
}

/// Returns the path to the mojang worlds directory.
fn mojang_worlds_dir(name: &str) -> Result<PathBuf, io::Error> {
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

/// Copies a directory recursively.
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
