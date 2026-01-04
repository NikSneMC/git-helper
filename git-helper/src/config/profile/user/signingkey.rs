use std::{
    fmt::{self, Display},
    str::FromStr,
};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

use crate::config::profile::keys::completion::PathCompletion;

#[derive(Deserialize, Serialize, Clone)]
pub struct UserSigningKey(pub String);
impl FromStr for UserSigningKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Display for UserSigningKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("user.signingkey: {}", self.0).fmt(f)
    }
}
impl UserSigningKey {
    pub fn input(default: Option<String>) -> dialoguer::Result<Option<Self>> {
        let input = Input::new()
            .with_prompt("Input the user.signingkey value")
            .with_initial_text(default.unwrap_or_default())
            .completion_with(&PathCompletion)
            .allow_empty(true)
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact()?;

        Ok((!input.is_empty()).then_some(Self(input)))
    }

    pub fn from_param(signing_key: Option<String>, default: Option<String>) -> Option<Self> {
        signing_key
            .filter(|signing_key| Self::from_str(signing_key).is_ok())
            .map(Self)
            .or_else(|| Self::input(default).unwrap())
    }
}
