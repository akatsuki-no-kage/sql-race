pub mod component;

use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
};
use widgetui::{
    constraint, layout, set, App, Chunks, Events, Res, ResMut, Set, WidgetFrame, WidgetResult,
};

use crate::state::{GlobalState, Screen};
use component::{rank, username_input};

pub fn chunk_generator(
    frame: Res<WidgetFrame>,
    mut chunks: ResMut<Chunks>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::Home {
        return Ok(());
    }

    let new_chunks = layout! {
        frame.size(),
        (%70) => { %25, %50, %25 },
        (#6) => { %25, %50, %25 }
    };

    chunks.register_chunk::<rank::Chunk>(new_chunks[0][1]);
    chunks.register_chunk::<username_input::Chunk>(new_chunks[1][1]);

    Ok(())
}

pub fn event_handler(mut events: ResMut<Events>, global_state: Res<GlobalState>) -> WidgetResult {
    if global_state.screen != Screen::Home {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => events.register_exit(),
        _ => {}
    }

    Ok(())
}

#[set]
pub fn HomeSet(app: App) -> App {
    app.states(rank::CustomState::default())
        .states(username_input::CustomState::default())
        .widgets(chunk_generator)
        .widgets(event_handler)
        .widgets(rank::render)
        .widgets(rank::state_updater)
        .widgets(username_input::render)
        .widgets(username_input::event_handler)
}
