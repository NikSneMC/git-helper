use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

use crate::config::profile::keys::{auth::AuthKey, sign::SignKey};

pub mod auth;
pub mod completion;
pub mod sign;

#[derive(Deserialize, Serialize)]
pub struct Keys {
    pub auth: AuthKey,
    pub sign: Option<SignKey>,
}
impl Display for Keys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines = vec![self.auth.to_string()];
        if let Some(sign) = &self.sign {
            lines.push(sign.to_string());
        }
        lines
            .iter()
            .map(|l| format!("- {l}"))
            .collect::<Vec<_>>()
            .join("\n")
            .fmt(f)
    }
}
