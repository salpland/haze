use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::error::Error;

/// Exports the given world to the `minecraftWorlds` directory.
pub fn export(name: &str, worlds: &[String], target: &str, overwrite: bool) -> Result<(), Error> {
    let from = local_worlds_dir(worlds, name)?;
    let to = mojang_worlds_dir(name, target)?;
    if to.exists() {
        if !overwrite {
            return Err(Error::CannotOverwriteWorld(name.to_string()));
        }
        delete_dir(&to)?;
    }
    copy_dir(&from, &to).map_err(|e| Error::CannotCopyWorld(e.kind(), name.to_string()))?;

    Ok(())
}

/// Imports the given world from the `minecraftWorlds` directory.
pub fn import(name: &str, worlds: &[String], target: &str) -> Result<(), Error> {
    let from = mojang_worlds_dir(name, target)?;
    let to: PathBuf = local_worlds_dir(worlds, name)?;
    if check_access(&from).is_err() {
        return Err(Error::CannotAccessDirectory(
            from.to_string_lossy().to_string(),
        ));
    }
    if to.exists() {
        delete_dir(&to)?;
    }
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
fn mojang_worlds_dir(name: &str, target: &str) -> Result<PathBuf, Error> {
    let target = match target {
        "stable" => "Microsoft.MinecraftUWP_8wekyb3d8bbwe",
        "preview" => "Microsoft.MinecraftWindowsBeta_8wekyb3d8bbwe",
        "education" => "Microsoft.MinecraftEducationEdition_8wekyb3d8bbwe",
        path => return Ok(PathBuf::from(path)),
    };

    env::var("LOCALAPPDATA")
        .map(|base| {
            PathBuf::from(&base)
                .join("Packages")
                .join(target)
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

/// Tries to access all files in the directory. Returns an error if access to any file is denied.
fn check_access(target: &Path) -> Result<(), io::Error> {
    check_access_helper(target)
}

fn check_access_helper(target: &Path) -> Result<(), io::Error> {
    for entry in fs::read_dir(target)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            check_access_helper(&entry.path())?;
        } else {
            // Open a file and immediately close it to check if it's accessible.
            fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(entry.path())?;
        }
    }
    Ok(())
}

/// Deletes a directory recursively. Before deleting files, it tries to open them first to check if
/// they are locked by another process. Only delets the directory if all files are accessible.
///
/// WARNING: This function is not atomic. It's possible that some other process will lock the file
/// after it's checked and before it's deleted.
fn delete_dir(target: &Path) -> Result<(), Error> {
    // Check if any of the files in the directory are locked by another process.
    if check_access(target).is_err() {
        return Err(Error::CannotDeleteDirectory(
            target.to_string_lossy().to_string(),
        ));
    }

    // Delete the directory
    if fs::remove_dir_all(target).is_err() {
        return Err(Error::CriticalDeleteDirectory(
            target.to_string_lossy().to_string(),
        ));
    };
    Ok(())
}
