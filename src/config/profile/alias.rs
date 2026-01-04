use std::{
    collections::hash_map::Keys,
    fmt::{self, Display},
};

use dialoguer::{Completion, Input};
use serde::{Deserialize, Serialize};

use crate::config::{Config, profile::Profile};

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ProfileAlias(pub String);

impl Display for ProfileAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("profile `{}`", self.0).fmt(f)
    }
}

impl ProfileAlias {
    pub fn from_str(s: &str) -> Result<Self, String> {
        if s.is_empty() {
            return Err("Profile name can't be empty".to_string());
        }
        Ok(Self(s.to_string()))
    }

    pub fn input(config: &Config) -> dialoguer::Result<Self> {
        Ok(Self(
            Input::new()
                .with_prompt("Input the profile name (alias)")
                .completion_with(&ProfileAliasCompletion::from(config))
                .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
                .interact_text()?,
        ))
    }

    pub fn from_param(alias: Option<String>, config: &Config) -> ProfileAlias {
        alias
            .filter(|alias| ProfileAlias::from_str(alias).is_ok())
            .map(ProfileAlias)
            .unwrap_or_else(|| ProfileAlias::input(config).unwrap())
    }
}

struct ProfileAliasCompletion<I>
where
    I: Iterator,
{
    options: I,
}

impl<'c> From<&'c Config> for ProfileAliasCompletion<Keys<'c, ProfileAlias, Profile>> {
    fn from(config: &'c Config) -> Self {
        Self {
            options: config.profiles.keys(),
        }
    }
}

impl<'i, I> Completion for ProfileAliasCompletion<I>
where
    I: Iterator<Item = &'i ProfileAlias> + Clone,
{
    fn get(&self, input: &str) -> Option<String> {
        let matches: Vec<_> = self
            .options
            .clone()
            .filter(|alias| alias.0.starts_with(input))
            .collect();

        if matches.len() == 1 {
            matches.first().map(|alias| alias.0.to_string())
        } else {
            None
        }
    }
}
