use crate::config::Config;
use lazy_static::lazy_static;
use std::path::PathBuf;

pub mod config;

lazy_static! {
    pub static ref HOME_DIR: PathBuf =
        dirs_next::home_dir().expect("Failed to obtain user's home directory");
    pub static ref CONFIG_DIR: PathBuf = HOME_DIR.join(".config");
}

pub fn parse_config() -> anyhow::Result<Config> {
    Config::load(&CONFIG_DIR.join("dotprofiles.toml")).or(Ok(Config::default()))
}
