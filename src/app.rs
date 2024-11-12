use anyhow::Result;
use sqlx::SqlitePool;

pub struct App {
    pub pool: SqlitePool,
    pub username: String,
    pub score: i64,
    pub exit: bool,
    pub state: AppState,
}

#[derive(PartialEq)]
pub enum AppState {
    Menu,
    InGame,
}

impl App {
    pub async fn new(username: String) -> Result<Self> {
        let pool = SqlitePool::connect("sqlite://score.db").await?;
        Ok(Self {
            pool,
            username,
            score: 0,
            exit: false,
            state: AppState::Menu,
        })
    }
}
