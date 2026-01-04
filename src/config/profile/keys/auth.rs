use std::{
    fmt::{self, Display},
    str::FromStr,
};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

use crate::config::profile::keys::completion::PathCompletion;

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthKey(pub String);
impl FromStr for AuthKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Display for AuthKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("key.auth: {}", self.0).fmt(f)
    }
}
impl AuthKey {
    pub fn input(default: Option<String>) -> dialoguer::Result<Self> {
        let input = Input::new()
            .with_prompt("Input the key.auth value")
            .with_initial_text(default.unwrap_or_default())
            .completion_with(&PathCompletion)
            .allow_empty(true)
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact_text()?;

        Ok(Self(input))
    }

    pub fn from_param(auth_key: Option<String>, default: Option<String>) -> Self {
        auth_key
            .filter(|auth_key| Self::from_str(auth_key).is_ok())
            .map(Self)
            .unwrap_or_else(|| Self::input(default).unwrap())
    }
}
