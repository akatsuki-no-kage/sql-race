use std::{path::PathBuf, sync::LazyLock};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub game_duration: u64,
    pub tick_rate: u64,
    pub database_file: String,
    pub question_pack_dir: PathBuf,
    pub question_count: usize,
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
