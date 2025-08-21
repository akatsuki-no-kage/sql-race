use std::fmt::Display;

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

impl Display for CustomState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.query.lines().join("\n"))
    }
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    mut state: ResMut<CustomState>,
    focus_state: Res<FocusState>,
) -> WidgetResult {
    let Ok(chunk) = chunks.get_chunk::<Chunk>() else {
        return Ok(());
    };

    let border_color = if focus_state.focused_element == ID {
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
    focus_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || focus_state.focused_element != ID {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    if let Event::Key(key_event) = event {
        state.query.input(*key_event);
    }

    Ok(())
}
