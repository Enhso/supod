use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub api: ApiConfig,
    pub time: TimeConfig,
    pub execution: ExecutionConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiConfig {
    pub podcastindex_key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TimeConfig {
    pub timezone_offset: i32,
    pub cutoff_hour: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExecutionConfig {
    pub default_weeks: u32,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        return Err("ERROR: Config - Could not find config.toml in current directory".into());
    }
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
