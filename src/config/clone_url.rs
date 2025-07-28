use std::str::FromStr;

use dialoguer::Input;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize)]
pub struct CloneUrl(pub String);
impl FromStr for CloneUrl {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Url::parse(s).map_err(|_| "Invalid repository url")?;

        Ok(Self(s.to_string()))
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
