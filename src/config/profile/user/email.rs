use std::{
    fmt::{self, Display},
    str::FromStr,
};

use dialoguer::Input;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserEmail(pub String);
impl FromStr for UserEmail {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !EmailAddress::is_valid(s) {
            return Err(format!("`{s}` is not a valid email address!"));
        }

        Ok(Self(s.to_string()))
    }
}
impl Display for UserEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("user.email: {}", self.0).fmt(f)
    }
}
impl UserEmail {
    pub fn input(default: Option<String>) -> dialoguer::Result<Self> {
        let input = Input::new()
            .with_prompt("Input the user.email value")
            .with_initial_text(default.unwrap_or_default())
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact()?;

        Ok(Self(input))
    }

    pub fn from_param(email: Option<String>, default: Option<String>) -> Self {
        email
            .filter(|email| Self::from_str(email).is_ok())
            .map(Self)
            .unwrap_or_else(|| Self::input(default).unwrap())
    }
}
