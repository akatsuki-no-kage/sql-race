mod components;
pub mod ingame_page;
mod ranking_page;

use anyhow::Result;
use ingame_page::InGamePage;
use ranking_page::RankingPage;
use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::{
    app::{App, AppState},
    controllers::check_exist_username,
    models::score::Score,
};

pub async fn init(app: &mut App) -> Result<()> {
    let db = app.pool.clone();
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut ranking_page = RankingPage::default();
    ranking_page.load_scores(&db).await?;
    let mut ingame = InGamePage::new();

    loop {
        match app.state {
            AppState::InGame => {
                ingame.update_states();
                ingame.update_question().await?;
                terminal.draw(|frame| {
                    frame.render_widget(&ingame, frame.area());
                })?;
                ingame.handle_key_events(app, &db).await?;
            }
            AppState::Menu => {
                terminal.draw(|frame| {
                    frame.render_widget(&ranking_page, frame.area());
                })?;

                // Handle input events for ranking page
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.exit = true;
                        }
                        KeyCode::Char(c) => {
                            ranking_page.update_input(format!("{}{}", ranking_page.input.value, c));
                        }
                        KeyCode::Backspace => {
                            if !ranking_page.input.value.is_empty() {
                                ranking_page.input.value.pop();
                            }
                        }
                        KeyCode::Enter => {
                            if check_exist_username(&db, ranking_page.input.value.clone()).await? {
                                ranking_page
                                    .set_error_message("Username already exists!".to_string());
                            } else {
                                app.username = ranking_page.input.value.clone();
                                Score::insert(&db, app.username.clone(), app.score).await?;
                                app.state = AppState::InGame;
                            };
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.exit {
            terminal.clear()?;
            ratatui::try_restore()?;
            return Ok(());
        }
    }
}
