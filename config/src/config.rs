use crate::HOME_DIR;
use anyhow::{Error, anyhow};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub log_level: Option<String>,
    pub package_manager: Option<String>,
    pub root_dir: Option<String>,
    pub scoop: Option<ScoopConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoopConfig {
    pub executable_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log_level: None,
            package_manager: None,
            root_dir: Some(HOME_DIR.join("dotprofiles").to_string_lossy().to_string()),
            scoop: None,
        }
    }
}

impl Config {
    pub fn load(config_path: &PathBuf) -> anyhow::Result<Config> {
        if !config_path.exists() {
            return Err(anyhow!("Configuration file does not exist"));
        } else if !config_path
            .as_path()
            .extension()
            .map_or(false, |ext| ext == "toml")
        {
            return Err(anyhow!(
                "Configuration file must end with the '.toml' extension"
            ));
        }
        let config_str = fs::read_to_string(config_path)?;
        toml::from_str(&config_str).map_err(|e| e.into())
    }

    pub fn get_app_dir(&self) -> Result<PathBuf, Error> {
        let root = self
            .root_dir
            .as_ref()
            .ok_or_else(|| anyhow!("Root directory is not set"))?;
        Ok(PathBuf::from(root).join("apps"))
    }
}
