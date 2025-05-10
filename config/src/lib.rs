use crate::config::Config;
use lazy_static::lazy_static;
use std::path::PathBuf;

pub mod config;

lazy_static! {
    pub static ref HomeDir: PathBuf =
        dirs_next::home_dir().expect("Failed to obtain user's home directory");
    pub static ref DefaultConfigFilePath: PathBuf =
        HomeDir.join(".config").join("dotprofiles.toml");
}

pub fn parse_config() -> Config {
    Config::load(&DefaultConfigFilePath).unwrap_or(Config::default())
}
