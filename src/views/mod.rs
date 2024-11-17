mod components;
pub mod ingame_page;
mod menu;

use anyhow::Result;
use ingame_page::InGamePage;
use menu::MenuPage;

use crate::app::{App, AppState};

pub async fn init(app: &mut App) -> Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;

    let mut menu = MenuPage::default();
    menu.load_scores(&app.pool).await?;
    let mut ingame = InGamePage::new();

    loop {
        match app.state {
            AppState::InGame => {
                ingame.update_states(app);
                ingame.update_question().await?;
                terminal.draw(|frame| {
                    frame.render_widget(&ingame, frame.area());
                })?;
                ingame.handle_key_events(app).await?;
            }
            AppState::Menu => {
                terminal.draw(|frame| {
                    frame.render_widget(&menu, frame.area());
                })?;
                menu.handle_key_events(app).await?;
            }
        }

        if app.exit {
            terminal.clear()?;
            ratatui::try_restore()?;
            return Ok(());
        }
    }
}
