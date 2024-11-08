mod components;
mod menu;
mod ranking_page;
mod gameplay;

use anyhow::Result;
use ranking_page::RankingPage;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use sqlx::SqlitePool;

pub async fn init(db: &SqlitePool) -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut ranking_page = RankingPage::default();
    ranking_page.load_scores(db).await?;

    loop {
        terminal.draw(|frame| {
            frame.render_widget(&ranking_page, frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match (key.modifiers, key.code) {
                    (_, KeyCode::Char('q')) => {
                        terminal.clear()?;
                        ratatui::try_restore()?;
                        return Ok(());
                    }
                    _ => println!("Nothing"),
                }
            }
        }
    }
}
