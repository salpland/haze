use serde::Deserialize;
use std::fs;

use crate::error::HazeResult;

#[derive(Deserialize)]
pub struct Config {
    pub packs: Packs,
}

#[derive(Deserialize)]
pub struct Packs {
    #[serde(rename = "worldTemplate")]
    pub world_template: String,
}

pub fn load(path: &String) -> HazeResult<Config> {
    let config = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config)?;

    Ok(config)
}
