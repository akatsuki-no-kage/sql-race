use std::{path::PathBuf, sync::LazyLock};

use serde::Deserialize;

const fn default_game_duration() -> u64 {
    5
}

const fn default_tick_rate() -> u64 {
    1
}

fn default_question_pack_dir() -> PathBuf {
    PathBuf::from("questions")
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_game_duration")]
    pub game_duration: u64,

    #[serde(default = "default_tick_rate")]
    pub tick_rate: u64,

    #[serde(default = "default_question_pack_dir")]
    pub question_pack_dir: PathBuf,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(::config::Environment::default().try_parsing(true))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});
