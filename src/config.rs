use colored::Colorize;
use serde::Deserialize;
use std::fs;

use crate::error::{HazeError, HazeResult};

#[derive(Deserialize)]
pub struct Config {
    pub packs: Packs,
}

#[derive(Deserialize)]
pub struct Packs {
    #[serde(rename = "worldTemplate")]
    pub world_template: String,
}

pub fn load(path: String) -> HazeResult<Config> {
    let config = fs::read_to_string(&path)
        .map_err(|e| HazeError::ConfigRead(e, path.clone().bold().underline()))?;
    let config: Config = serde_json::from_str(&config)
        .map_err(|e| HazeError::ConfigParse(e, path.bold().underline()))?;

    Ok(config)
}
