use std::{
    env,
    fmt::{self, Display},
};

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
    pub fn apply(&self) -> Result<(), git2::Error> {
        let current_dir = env::current_dir().expect("Current dir to be valid");
        let mut config = git2::Repository::open(current_dir)
            .expect("Current folder to be a valid git repository")
            .config()
            .expect("Repo config to be available");

        config.set_str("user.name", &self.user.name.0)?;
        config.set_str("user.email", &self.user.email.0)?;
        config.set_str("core.sshCommand", &format!("ssh -i {}", &self.keys.auth.0))?;

        if let Some(sign_key) = &self.keys.sign {
            config.set_str("user.signingkey", &sign_key.0)?;
        }
        Ok(())
    }
}
