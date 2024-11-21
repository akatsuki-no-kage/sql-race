use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    widgets::{Paragraph, Widget},
};
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::state::{GlobalState, Screen};

pub fn handle_key(
    mut global_state: ResMut<GlobalState>,
    events: Res<Events>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
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
        }) => global_state.screen = Screen::Home,
        _ => {}
    }

    Ok(())
}

pub struct InGame<'a> {
    pub global_state: Res<'a, GlobalState>,
}

impl Widget for InGame<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.global_state.username.lines().join("\n")).render(area, buf)
    }
}
