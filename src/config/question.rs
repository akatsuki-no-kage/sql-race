use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    Easy,
    Medium,
    Hard,
}

impl From<Level> for &'static str {
    fn from(level: Level) -> Self {
        match level {
            Level::Easy => "easy",
            Level::Medium => "medium",
            Level::Hard => "hard",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QuestionConfig {
    pub root: PathBuf,
    pub count: HashMap<Level, usize>,
}
