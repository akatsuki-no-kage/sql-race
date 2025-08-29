use crate::{config::CONFIG, repository::ScoreRepository};

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Screen {
    #[default]
    Home,
    Game,
}

pub struct AppState {
    pub name: Option<String>,
    pub screen: Screen,
    pub score_repository: ScoreRepository,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            name: Default::default(),
            screen: Default::default(),
            score_repository: ScoreRepository::new(&CONFIG.database_file).unwrap(),
        }
    }
}
