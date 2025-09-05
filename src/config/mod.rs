mod game;
mod question;

use std::sync::LazyLock;

use serde::Deserialize;

use crate::config::{game::GameConfig, question::QuestionConfig};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub game: GameConfig,
    pub question: QuestionConfig,
    pub database_file: String,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?
            .try_deserialize()
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new().unwrap());
