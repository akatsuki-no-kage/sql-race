use std::time::{Duration, Instant};

use anyhow::Result;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    widgets::{Paragraph, Widget},
};
use tui_textarea::TextArea;
use widgetui::{Events, Res, ResMut, State, WidgetResult};

use crate::{
    model::Question,
    state::{GlobalState, Screen},
    util,
};

const TIME_LIMIT: Duration = Duration::from_secs(100);

#[derive(State)]
pub struct InGameState {
    query: TextArea<'static>,
    score: usize,
    time_start: Instant,
    is_schema_table_visible: bool,
    focused_element: usize,
    question: Question,
    question_index: usize,
    execution_option: usize,
    is_done: bool,
}

impl InGameState {
    pub async fn default() -> Result<Self> {
        Ok(Self {
            query: Default::default(),
            score: Default::default(),
            time_start: Instant::now(),
            is_schema_table_visible: Default::default(),
            focused_element: Default::default(),
            question: util::get_question(1).await?,
            question_index: 1,
            execution_option: Default::default(),
            is_done: Default::default(),
        })
    }
}

pub struct InGame<'a> {
    pub in_game_state: Res<'a, InGameState>,
    pub global_state: Res<'a, GlobalState>,
}

impl Widget for InGame<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        Paragraph::new(self.global_state.username.lines().join("\n")).render(area, buf)
    }
}

pub fn event_handler(events: Res<Events>, mut global_state: ResMut<GlobalState>) -> WidgetResult {
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
