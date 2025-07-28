use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::config::profile::credential::username::CredentialUsername;

pub mod username;

#[derive(Deserialize, Serialize)]
pub struct Credential {
    pub username: Option<CredentialUsername>,
}
impl Display for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![];
        if let Some(username) = &self.username {
            lines.push(username.to_string());
        }
        lines
            .iter()
            .map(|l| format!("- {l}"))
            .collect::<Vec<_>>()
            .join("\n")
            .fmt(f)
    }
}
