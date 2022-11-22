use serde::Deserialize;
use std::fs;

use crate::error::{HazeError, HazeResult};

#[derive(Deserialize)]
pub struct Config {
    pub worlds: Vec<String>,
}

pub fn load(path: String) -> HazeResult<Config> {
    let config = fs::read_to_string(&path)
        .map_err(|e| HazeError::CannotReadConfig(e.kind(), path.clone()))?;
    let config: Config =
        serde_json::from_str(&config).map_err(|e| HazeError::CannotParseConfig(e, path))?;

    Ok(config)
}
