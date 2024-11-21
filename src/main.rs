#![feature(iterator_try_collect)]

pub mod compoment;
pub mod model;
pub mod page;
pub mod state;
pub mod util;

use std::sync::Arc;

use anyhow::Result;
use page::{
    home::{self, component::*, Home},
    in_game::{self, InGame},
};
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use sqlx::SqlitePool;
use state::{GlobalState, Screen};
use widgetui::{App, Events, Res, ResMut, WidgetFrame, WidgetResult};

fn render(
    mut frame: ResMut<WidgetFrame>,
    global_state: Res<GlobalState>,
    home_state: Res<home::HomeState>,
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
            frame.render_widget(InGame { global_state }, area);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Arc::new(SqlitePool::connect("sqlite:score.db").await?);
    let global_state = GlobalState::new(pool.clone());
    let home_state = home::HomeState::default();

    App::new(60)?
        .states(global_state)
        .states(home_state)
        .widgets(render)
        .widgets(home::event_handler)
        .widgets(home::component::rank::state_updater)
        .widgets(home::component::username_input::event_handler)
        .widgets(in_game::event_handler)
        .run()?;
    Ok(())
}
