use crate::DefaultConfigFilePath;
use anyhow::{Error, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub log_level: Option<String>,
    pub package_manager: Option<String>,
    pub scoop: Option<ScoopConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoopConfig {
    pub executable_path: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            log_level: None,
            package_manager: None,
            scoop: None,
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

pub fn parse_config() -> Config {
    Config::load(&DefaultConfigFilePath).unwrap_or(Config::default())
}
