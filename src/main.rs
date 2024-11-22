#![feature(iterator_try_collect)]

pub mod model;
pub mod page;
pub mod state;
pub mod util;

use std::sync::Arc;

use anyhow::Result;
use page::{
    home::{self, Home, HomeState},
    in_game::{self, InGame, InGameState},
};
use sqlx::SqlitePool;
use state::{GlobalState, Screen};
use widgetui::{App, Res, ResMut, WidgetFrame, WidgetResult};

fn render(
    mut frame: ResMut<WidgetFrame>,
    global_state: Res<GlobalState>,
    home_state: Res<HomeState>,
    in_game_state: ResMut<InGameState>,
) -> WidgetResult {
    let area = frame.size();

    match global_state.screen {
        Screen::Home => {
            frame.render_widget(
                Home {
                    global_state,
                    home_state,
                },
                area,
            );
        }
        Screen::InGame => {
            frame.render_widget(
                InGame {
                    global_state,
                    in_game_state,
                },
                area,
            );
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Arc::new(SqlitePool::connect("sqlite:score.db").await?);
    let global_state = GlobalState::new(pool.clone());
    let home_state = home::HomeState::default();
    let in_game_state = InGameState::default().await?;

    App::new(60)?
        .states(global_state)
        .states(home_state)
        .states(in_game_state)
        .widgets(render)
        .widgets(home::event_handler)
        .widgets(home::component::rank::state_updater)
        .widgets(home::component::username_input::event_handler)
        .widgets(in_game::event_handler)
        .widgets(in_game::component::query_input::event_handler)
        .widgets(in_game::component::action::event_handler)
        .widgets(in_game::component::schema::event_handler)
        .run()?;
    Ok(())
}
