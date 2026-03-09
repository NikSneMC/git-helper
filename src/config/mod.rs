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
    pub base_dir: PathBuf,
    pub profiles: HashMap<ProfileAlias, Profile>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: crate_version!().to_string(),
            base_dir: home_dir()
                .context("while getting the home dir")
                .unwrap()
                .join(Self::DEFAULT_PROJECTS_DIR_NAME),
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
    const DEFAULT_PROJECTS_DIR_NAME: &'static str = "Projects";
    const CONFIG_NAME: &'static str = ".git-helper.toml";

    pub fn path() -> Result<PathBuf> {
        Ok(home_dir()
            .context("while getting home dir")?
            .join(Self::CONFIG_NAME))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::path()?;
        let contents = toml::to_string_pretty(&self).context("while serializing config")?;
        fs::write(&config_path, contents).context("while writing to the config file")?;
        Ok(())
    }

    pub fn init() -> Result<()> {
        Self::default()
            .save()
            .context("while saving the default config")?;
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::path()?;
        if !config_path.exists() {
            Self::init().context("while initing config")?;
        }
        let config_str = fs::read_to_string(&config_path).context("Failed to read config file")?;

        toml::from_str::<Self>(config_str.leak()).context("Failed to parse config")
    }
}
