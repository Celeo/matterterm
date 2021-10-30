//! On-disk file configuration.

use crate::globals::CONFIG_FILE_NAME;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// Struct representing the on-disk program configuration.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub email: String,
    pub password: String,
    pub enable_file_logging: bool,
}

impl Config {
    /// Get the path to the config file on the disk.
    ///
    /// Can fail if the user's config directory cannot be resolved.
    pub fn get_path() -> Result<PathBuf> {
        Ok(directories::BaseDirs::new()
            .ok_or_else(|| anyhow!("Could not determine user config directory"))?
            .config_dir()
            .join("matterterm")
            .join(CONFIG_FILE_NAME))
    }

    /// Whether the config file exists on the disk.
    ///
    /// Can fail if the user's config directory cannot be resolved.
    pub fn exists() -> Result<bool> {
        Ok(Config::get_path()?.exists())
    }

    /// Load the configuration from the disk.
    ///
    /// Can fail if the user's config directory cannot be resolved
    /// or the file does not exist (use `Config::exists()`).
    pub fn load_from_disk() -> Result<Self> {
        let config = serde_json::from_str(&fs::read_to_string(Config::get_path()?)?)?;
        Ok(config)
    }

    /// Creates a new, empty `Config` struct and writes it to
    /// the disk, overwriting any file that may exist (use `Config::exists()`).
    ///
    /// Can fail if the user's config directory cannot be resolved or
    /// directory/file permissions prevent the file from being written to.
    pub fn create_new() -> Result<Self> {
        let config = Config::default();
        fs::write(Config::get_path()?, serde_json::to_string_pretty(&config)?)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    //
}
