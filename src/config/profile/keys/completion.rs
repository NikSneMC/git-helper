use std::{
    fs,
    path::{Path, MAIN_SEPARATOR},
};

use dialoguer::Completion;
use dirs::home_dir;

pub struct PathCompletion;

impl Completion for PathCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let path = Path::new(input);
        let (dir, prefix) = if input.ends_with(MAIN_SEPARATOR) {
            (path, "")
        } else {
            (
                path.parent().unwrap_or_else(|| Path::new(".")),
                path.file_name().and_then(|s| s.to_str()).unwrap_or(""),
            )
        };

        let expanded_dir = if dir.starts_with("~") {
            if let Ok(stripped) = dir.strip_prefix("~") {
                home_dir()
                    .map(|h| h.join(stripped))
                    .unwrap_or_else(|| dir.to_path_buf())
            } else if dir == Path::new("~") {
                home_dir().unwrap_or_else(|| dir.to_path_buf())
            } else {
                dir.to_path_buf()
            }
        } else {
            dir.to_path_buf()
        };

        let entries = fs::read_dir(&expanded_dir).ok()?;
        let matches: Vec<String> = entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let mut name = entry.file_name().to_string_lossy().to_string();
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    name.push(MAIN_SEPARATOR);
                }
                name
            })
            .filter(|name| name.starts_with(prefix))
            .collect();

        if matches.is_empty() {
            return None;
        }

        let common_prefix = matches.iter().fold(matches[0].clone(), |acc, x| {
            acc.chars()
                .zip(x.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        });

        let result = if dir == Path::new(".") {
            common_prefix
        } else {
            dir.join(&common_prefix).to_string_lossy().to_string()
        };

        Some(result)
    }
}
