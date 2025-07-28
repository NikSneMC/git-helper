use std::{
    fmt::{self, Display},
    str::FromStr,
};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct CredentialUsername(pub String);
impl FromStr for CredentialUsername {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Display for CredentialUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("credential.username: {}", self.0).fmt(f)
    }
}
impl CredentialUsername {
    pub fn input(default: Option<String>) -> dialoguer::Result<Option<Self>> {
        let input = Input::new()
            .with_prompt("Input the credential.username value")
            .with_initial_text(default.unwrap_or_default())
            .allow_empty(true)
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact()?;

        Ok((!input.is_empty()).then_some(Self(input)))
    }

    pub fn from_param(username: Option<String>, default: Option<String>) -> Option<Self> {
        username
            .filter(|username| Self::from_str(username).is_ok())
            .map(Self)
            .or_else(|| Self::input(default).unwrap())
    }
}
