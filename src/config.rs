use std::path::PathBuf;

use serde::Deserialize;

const fn default_game_duration() -> u64 {
    5
}

const fn default_tick_rate() -> u64 {
    1
}

fn default_database_file() -> String {
    "score.db".to_string()
}

fn default_question_pack_dir() -> PathBuf {
    PathBuf::from("questions")
}

const fn default_question_count() -> usize {
    10
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_game_duration")]
    pub game_duration: u64,

    #[serde(default = "default_tick_rate")]
    pub tick_rate: u64,

    #[serde(default = "default_database_file")]
    pub database_file: String,

    #[serde(default = "default_question_pack_dir")]
    pub question_pack_dir: PathBuf,

    #[serde(default = "default_question_count")]
    pub question_count: usize,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default().try_parsing(true))
            .build()?
            .try_deserialize()
    }
}
