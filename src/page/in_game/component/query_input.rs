use ratatui::{
    crossterm::event::{Event, KeyModifiers},
    prelude::*,
};
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::{
    page::in_game::InGameState,
    state::{GlobalState, Screen},
};

pub const ID: usize = 0;

pub struct QueryInput<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for QueryInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.in_game_state.query.render(area, buf);
    }
}

pub fn event_handler(
    events: Res<Events>,
    mut in_game_state: ResMut<InGameState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || in_game_state.focused_element != ID {
        return Ok(());
    }

    let Some(event) = &events.event else {
        return Ok(());
    };

    match event {
        Event::Key(key_event) if key_event.modifiers == KeyModifiers::NONE => {
            in_game_state.query.input(*key_event);
        }
        _ => {}
    }

    Ok(())
}
