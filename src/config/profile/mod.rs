use std::{
    env,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

use crate::config::profile::{credential::Credential, user::User};

pub mod alias;
pub mod credential;
pub mod user;

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub user: User,
    pub credential: Credential,
}
impl Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        [self.user.to_string(), self.credential.to_string()]
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

        if let Some(signingkey) = &self.user.signingkey {
            config.set_str("user.signingkey", &signingkey.0)?;
        }
        if let Some(username) = &self.credential.username {
            config.set_str("credential.username", &username.0)?;
        }
        Ok(())
    }
}
