use ratatui::{
    crossterm::event::Event,
    prelude::*,
    widgets::{Block, Borders},
};
use tui_textarea::TextArea;
use widgetui::{Chunks, Events, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    page::in_game::FocusState,
    state::{GlobalState, Screen},
};

pub const ID: usize = 0;

pub struct Chunk;

#[derive(Default, State)]
pub struct CustomState {
    pub query: TextArea<'static>,
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    mut state: ResMut<CustomState>,
    in_game_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    let border_color = if in_game_state.focused_element == ID {
        Color::Green
    } else {
        Color::White
    };

    state.query.set_block(
        Block::default()
            .title("Query")
            .borders(Borders::ALL)
            .fg(border_color),
    );

    state.query.render(chunk, frame.buffer_mut());

    Ok(())
}

pub fn event_handler(
    events: Res<Events>,
    mut state: ResMut<CustomState>,
    in_game_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || in_game_state.focused_element != ID {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(key_event) => {
            state.query.input(*key_event);
        }
        _ => {}
    }

    Ok(())
}
