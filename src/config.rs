use std::sync::LazyLock;

use serde::Deserialize;

const fn default_game_duration() -> u64 {
    180 * 1000
}

const fn default_tick_rate() -> u64 {
    1000
}

const fn default_event_time_out() -> u64 {
    10
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_game_duration")]
    pub game_duration: u64,

    #[serde(default = "default_tick_rate")]
    pub tick_rate: u64,

    #[serde(default = "default_event_time_out")]
    pub event_time_out: u64,
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    ::config::Config::builder()
        .add_source(::config::Environment::default().try_parsing(true))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap()
});
