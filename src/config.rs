use std::sync::LazyLock;

use serde::Deserialize;
use strum::IntoStaticStr;

#[derive(Debug, PartialEq, Eq, Hash, Deserialize, IntoStaticStr)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Mode {
    Practice,
    Challenge,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub game_duration: u64,
    pub tick_rate: u64,
    pub mode: Mode,
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
