use std::fmt::{self, Display};

use dialoguer::Input;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ProfileAlias(pub String);
impl Display for ProfileAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("profile `{}`", self.0).fmt(f)
    }
}
impl ProfileAlias {
    pub fn from_str(s: &str, config: &Config, check: bool) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Profile name couldn't be empty".to_string());
        }
        if check && config.profiles.contains_key(&Self(s.to_string())) {
            return Err(format!("Profile with name `{s}` already exists"));
        }
        Ok(Self(s.to_string()))
    }

    pub fn input(config: &Config, check: bool) -> dialoguer::Result<Self> {
        Ok(Self(
            Input::new()
                .with_prompt("Input the profile name (alias)")
                .validate_with(|input: &String| Self::from_str(input, config, check).map(|_| ()))
                .interact()?,
        ))
    }

    pub fn from_param(alias: Option<String>, config: &Config, check: bool) -> ProfileAlias {
        alias
            .filter(|alias| ProfileAlias::from_str(alias, config, check).is_ok())
            .map(ProfileAlias)
            .unwrap_or_else(|| ProfileAlias::input(config, check).unwrap())
    }
}
