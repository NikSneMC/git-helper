use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::config::profile::user::{email::UserEmail, name::UserName};

pub mod email;
pub mod name;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: UserName,
    pub email: UserEmail,
}
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = vec![self.name.to_string(), self.email.to_string()];
        lines
            .iter()
            .map(|l| format!("- {l}"))
            .collect::<Vec<_>>()
            .join("\n")
            .fmt(f)
    }
}
