use crate::{config::CONFIG, repository::ScoreRepository};

pub struct OwnStates {
    repository: ScoreRepository,
}

impl Default for OwnStates {
    fn default() -> Self {
        Self {
            repository: ScoreRepository::new(&CONFIG.database_file).unwrap(),
        }
    }
}
