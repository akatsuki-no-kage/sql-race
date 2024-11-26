#![feature(iterator_try_collect)]

pub mod model;
pub mod page;
pub mod state;
pub mod util;

use std::sync::Arc;

use anyhow::Result;
use page::{home::HomeSet, in_game::InGameSet, schema::SchemaSet};
use sqlx::SqlitePool;
use state::GlobalState;
use widgetui::App;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = Arc::new(SqlitePool::connect("sqlite:score.db").await?);
    let global_state = GlobalState::new(pool.clone());

    App::new(60)?
        .states(global_state)
        .sets(HomeSet)
        .sets(InGameSet)
        .sets(SchemaSet)
        .run()?;
    Ok(())
}
