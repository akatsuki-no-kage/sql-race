mod components;
pub mod ingame_page;
mod menu;
mod ranking_page;

use anyhow::Result;
use ingame_page::InGamePage;
use ranking_page::RankingPage;

use sqlx::SqlitePool;

use crate::app::{App, AppState};

pub async fn init(db: &SqlitePool, app: &mut App) -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut ranking_page = RankingPage::default();
    ranking_page.load_scores(db).await?;

    let mut ingame = InGamePage::new();

    loop {
        match app.state {
            AppState::Menu => (),
            AppState::InGame => {
                ingame.update_states();
                ingame.update_question().await?;
                terminal.draw(|frame| {
                    frame.render_widget(&ingame, frame.area());
                })?;
                ingame.handle_key_events(app, db).await?;
            }
        }

        if app.exit {
            terminal.clear()?;
            ratatui::try_restore()?;
            return Ok(());
        }
    }
}
