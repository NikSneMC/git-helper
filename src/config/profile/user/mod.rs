use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::config::profile::user::{email::UserEmail, name::UserName, signingkey::UserSigningKey};

pub mod email;
pub mod name;
pub mod signingkey;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: UserName,
    pub email: UserEmail,
    pub signingkey: Option<UserSigningKey>,
}
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![self.name.to_string(), self.email.to_string()];
        if let Some(signing_key) = &self.signingkey {
            lines.push(signing_key.to_string());
        }
        lines
            .iter()
            .map(|l| format!("- {l}"))
            .collect::<Vec<_>>()
            .join("\n")
            .fmt(f)
    }
}
