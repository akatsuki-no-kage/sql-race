#![feature(iterator_try_collect)]

pub mod model;
pub mod page;
pub mod state;
pub mod util;

use std::sync::Arc;

use anyhow::Result;
use page::{home::HomeSet, in_game::{self, InGame, InGameState}};
use sqlx::SqlitePool;
use state::{GlobalState, Screen};
use widgetui::{App, Res, ResMut, WidgetFrame, WidgetResult};

fn render(
    mut frame: ResMut<WidgetFrame>,
    global_state: Res<GlobalState>,
    in_game_state: ResMut<InGameState>,
) -> WidgetResult {
    let area = frame.size();

    match global_state.screen {
        Screen::InGame => {
            frame.render_widget(
                InGame {
                    global_state,
                    in_game_state,
                },
                area,
            );
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Arc::new(SqlitePool::connect("sqlite:score.db").await?);
    let global_state = GlobalState::new(pool.clone());
    let in_game_state = InGameState::default().await?;

    App::new(60)?
        .states(global_state)
        .states(in_game_state)
        .sets(HomeSet)
        .widgets(render)
        .widgets(in_game::event_handler)
        .widgets(in_game::state_updater)
        .widgets(in_game::component::query_input::event_handler)
        .widgets(in_game::component::action::event_handler)
        .widgets(in_game::component::schema::event_handler)
        .widgets(in_game::component::table::event_handler)
        .run()?;
    Ok(())
}
