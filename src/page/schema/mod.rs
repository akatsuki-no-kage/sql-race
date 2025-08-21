use component::schema;
use ratatui::layout::{Constraint, Direction, Layout};
use widgetui::{constraint, layout, set, App, Chunks, Res, ResMut, Set, WidgetFrame, WidgetResult};

use crate::state::{GlobalState, Screen};

pub mod component;

pub fn chunk_generator(
    frame: Res<WidgetFrame>,
    mut chunks: ResMut<Chunks>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::Schema {
        return Ok(());
    }

    let chunk = layout! {
        frame.size(),
        (%20),
        (%60) => { %20, %60, %20 },
        (%20)
    }[1][1];

    chunks.register_chunk::<schema::Chunk>(chunk);

    Ok(())
}

#[set]
pub fn SchemaSet(app: App) -> App {
    app.states(schema::CustomState::default()).widgets((
        chunk_generator,
        schema::render,
        schema::event_handler,
    ))
}
