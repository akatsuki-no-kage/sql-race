use std::sync::Arc;

use sqlx::SqlitePool;
use widgetui::State;

#[derive(Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Home,
    InGame,
}

#[derive(State)]
pub struct GlobalState {
    pub pool: Arc<SqlitePool>,
    pub username: String,
    pub score: u32,
    pub screen: Screen,
}

impl GlobalState {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            pool,
            username: String::new(),
            score: 0,
            screen: Default::default(),
        }
    }
}
