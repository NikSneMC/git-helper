use std::fmt::{self, Display};

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
        vec![self.user.to_string(), self.credential.to_string()]
            .join("\n")
            .fmt(f)
    }
}
