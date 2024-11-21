use std::sync::Arc;

use ratatui::{
    layout::Alignment,
    widgets::{Block, Borders},
};
use sqlx::SqlitePool;
use tui_textarea::TextArea;
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
    pub username: TextArea<'static>,
    pub score: u32,
    pub screen: Screen,
}

impl GlobalState {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        let mut text_area = TextArea::default();
        text_area.set_placeholder_text("Name here");
        text_area.set_block(Block::default().borders(Borders::ALL).title("Name"));
        text_area.set_alignment(Alignment::Left);

        Self {
            pool,
            username: text_area,
            score: 0,
            screen: Default::default(),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.lines().join("\n")
    }
}
