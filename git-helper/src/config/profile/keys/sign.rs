use std::{
    fmt::{self, Display},
    str::FromStr,
};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

use crate::config::profile::keys::completion::PathCompletion;

#[derive(Deserialize, Serialize, Clone)]
pub struct SignKey(pub String);
impl FromStr for SignKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Display for SignKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("key.sign: {}", self.0).fmt(f)
    }
}
impl SignKey {
    pub fn input(default: Option<String>) -> dialoguer::Result<Option<Self>> {
        let input = Input::new()
            .with_prompt("Input the key.sign value")
            .with_initial_text(default.unwrap_or_default())
            .completion_with(&PathCompletion)
            .allow_empty(true)
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact()?;

        Ok((!input.is_empty()).then_some(Self(input)))
    }

    pub fn from_param(sign_key: Option<String>, default: Option<String>) -> Option<Self> {
        sign_key
            .filter(|sign_key| Self::from_str(sign_key).is_ok())
            .map(Self)
            .or_else(|| Self::input(default).unwrap())
    }
}
