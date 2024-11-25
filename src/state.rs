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
    pub screen: Screen,
}

impl GlobalState {
    pub fn new(pool: Arc<SqlitePool>) -> Self {

        Self {
            pool,
            screen: Default::default(),
        }
    }
}
