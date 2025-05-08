use anyhow::{Error, anyhow};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub log_level: String,
}

impl Config {
    pub fn default_config() -> Config {
        Config {
            log_level: LevelFilter::Info.to_string(),
        }
    }

    pub fn load(config_path: &PathBuf) -> Result<Config, Error> {
        if !config_path.exists() {
            return Err(anyhow!("配置文件不存在"));
        } else if !config_path
            .as_path()
            .extension()
            .map_or(false, |ext| ext == "toml")
        {
            return Err(anyhow!("配置文件必须以 .toml 扩展名结尾"));
        }
        let config_str = fs::read_to_string(config_path)?;
        toml::from_str(&config_str).map_err(|e| e.into())
    }
}
