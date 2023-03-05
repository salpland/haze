use serde::Deserialize;
use std::fs;

use haze_core::Error;

#[derive(Deserialize)]
pub struct Config {
    pub worlds: Vec<String>,
}

pub fn load(path: String) -> Result<Config, Error> {
    let config =
        fs::read_to_string(&path).map_err(|e| Error::CannotReadConfig(e.kind(), path.clone()))?;
    let config = serde_json::from_str(&config).map_err(|e| Error::CannotParseConfig(e, path))?;

    Ok(config)
}
