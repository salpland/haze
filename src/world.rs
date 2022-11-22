use colored::Colorize;
use glob::glob;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use crate::error::{HazeError, HazeResult};

pub fn test(name: String, worlds: Vec<String>, overwrite: bool) -> HazeResult<()> {
    let from: PathBuf = local_worlds_dir(worlds, name.clone())?;
    let to = mojang_worlds_dir(&name).map_err(HazeError::LocalAppData)?;

    if to.exists() && !overwrite {
        Err(HazeError::OverwriteWorld(name.bold().underline()))?;
    }

    copy_dir(from, to).map_err(|e| HazeError::WorldCopy(e, name.to_string().bold().underline()))?;

    if overwrite {
        println!(
            "Updated '{}' in '{}' ({})",
            name,
            "minecraftWorlds".bold(),
            "overwrite".red()
        );
    } else {
        println!(
            "Copied world '{}' to '{}' for testing",
            "minecraftWorlds".bold(),
            name
        );
    }
    Ok(())
}

pub fn save(name: String, worlds: Vec<String>) -> HazeResult<()> {
    let from = mojang_worlds_dir(&name).map_err(HazeError::LocalAppData)?;
    let to: PathBuf = local_worlds_dir(worlds, name.clone())?;

    copy_dir(from, to).map_err(|e| HazeError::WorldCopy(e, name.to_string().bold().underline()))?;

    println!("Saved world '{}' to local worlds directory", name.bold());
    Ok(())
}

fn local_worlds_dir(worlds_patterns: Vec<String>, name: String) -> HazeResult<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for pattern in worlds_patterns.clone() {
        for path in glob(&pattern).map_err(|e| HazeError::InvalidWorldsGlobPattern(e, pattern))? {
            match path {
                Ok(path) => paths.push(path),
                Err(e) => {
                    return Err(HazeError::WorldsDirectoryNotFound(
                        e.error().kind(),
                        worlds_patterns,
                    ))
                }
            }
        }
    }

    if paths.is_empty() {
        return Err(HazeError::EmptyWorldsProperty);
    }
    let Some(path) = paths.iter().find(|p| p.ends_with(&name)) else {
        return Err(HazeError::WorldNotFound(paths, name));
    };

    Ok(path.clone())
}

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
