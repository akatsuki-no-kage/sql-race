use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub duration: u64,
    pub tick_rate: u64,
}
