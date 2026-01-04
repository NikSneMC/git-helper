use std::{
    fmt::{self, Display},
    str::FromStr,
};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserName(pub String);
impl FromStr for UserName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
impl Display for UserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("user.name: {}", self.0).fmt(f)
    }
}
impl UserName {
    pub fn input(default: Option<String>) -> dialoguer::Result<Self> {
        let input = Input::new()
            .with_prompt("Input the user.name value")
            .with_initial_text(default.unwrap_or_default())
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact_text()?;

        Ok(Self(input))
    }

    pub fn from_param(name: Option<String>, default: Option<String>) -> Self {
        name.filter(|name| Self::from_str(name).is_ok())
            .map(Self)
            .unwrap_or_else(|| Self::input(default).unwrap())
    }
}
