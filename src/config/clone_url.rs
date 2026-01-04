use std::str::FromStr;

use dialoguer::Input;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CloneUrl(pub String);
impl FromStr for CloneUrl {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ssh_regex = Regex::new(
            r"^(?:ssh:\/\/)?[a-zA-Z0-9._-]+@[a-zA-Z0-9._-]+(?::|\/)[a-zA-Z0-9._\/-]+\.git$",
        )
        .unwrap();

        ssh_regex
            .is_match(s)
            .then_some(Self(s.to_string()))
            .ok_or("Only ssh repository references are supported")
    }
}
impl CloneUrl {
    pub fn input() -> dialoguer::Result<Self> {
        let input = Input::new()
            .with_prompt("Input the repo url")
            .validate_with(|input: &String| Self::from_str(input).map(|_| ()))
            .interact()?;

        Ok(Self(input))
    }

    pub fn from_param(name: Option<String>) -> Self {
        name.filter(|name| Self::from_str(name).is_ok())
            .map(Self)
            .unwrap_or_else(|| Self::input().unwrap())
    }
}
