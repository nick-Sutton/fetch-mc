use serde::Deserialize;
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mods: Option<Vec<String>>,
    pub resourcepacks: Option<Vec<String>>,
    pub shaders: Option<Vec<String>>,
}

pub fn parse_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)?;
    return Ok(toml::from_str(&content)?);
}