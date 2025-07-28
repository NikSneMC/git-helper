use std::{
    collections::HashMap,
    env,
    fmt::{self, Debug, Display},
    fs,
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::crate_version;
use dirs::home_dir;
use serde::{Deserialize, Serialize};

pub mod clone_url;
pub mod profile;

use profile::Profile;

use crate::config::profile::alias::ProfileAlias;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub profiles: HashMap<ProfileAlias, Profile>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: crate_version!().to_string(),
            profiles: HashMap::default(),
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        toml::to_string_pretty(self).fmt(f)
    }
}

impl Config {
    const CONFIG_NAME: &'static str = ".git-helper.toml";

    pub fn path() -> Result<PathBuf> {
        Ok(home_dir()
            .context("Expected home_dir to be Some")?
            .join(Self::CONFIG_NAME))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::path()?;
        let contents = toml::to_string_pretty(&self).context("Failed to serialize config")?;
        fs::write(&config_path, contents).context("Failed to save config")?;
        Ok(())
    }

    pub fn init() -> Result<()> {
        Self::default().save()?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::path()?;
        if !config_path.exists() {
            Self::init()?;
        }
        let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;
        let config = toml::from_str::<Self>(config_str.leak()).context("Failed to parse config")?;
        Ok(config)
    }
}
