use std::{
    fmt::{self, Display},
    path::Path,
};

use anyhow::{Context, Result};
use git2::Repository;
use serde::{Deserialize, Serialize};

use crate::config::profile::{keys::Keys, user::User};

pub mod alias;
pub mod keys;
pub mod user;

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub user: User,
    pub keys: Keys,
}
impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        [self.user.to_string(), self.keys.to_string()]
            .join("\n")
            .fmt(f)
    }
}
impl Profile {
    pub fn apply_at(&self, path: &Path) -> Result<()> {
        let mut config = Repository::open(path)
            .context("while opening the repository")?
            .config()
            .context("while getting repo config")?;

        config.set_str("user.name", &self.user.name.0)?;
        config.set_str("user.email", &self.user.email.0)?;
        config.set_str("core.sshCommand", &format!("ssh -i {}", self.keys.auth.0))?;

        if let Some(sign_key) = &self.keys.sign {
            config.set_str("user.signingkey", &sign_key.0)?;
        }
        Ok(())
    }

    pub fn apply(&self) -> Result<()> {
        let current_dir =
            std::env::current_dir().context("while getting current directory")?;
        self.apply_at(&current_dir)
    }
}
