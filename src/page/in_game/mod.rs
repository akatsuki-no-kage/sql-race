pub mod component;

use std::time::{Duration, Instant};

use anyhow::Result;
use component::{
    hotkey_guide::HotKeyGuild, query_input::QueryInput, question::Question, score::Score,
    timer::Timer,
};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    widgets::Widget,
};
use tui_textarea::TextArea;
use widgetui::{Events, Res, ResMut, State, WidgetResult};

use crate::{
    model,
    state::{GlobalState, Screen},
    util,
};

const TIME_LIMIT: Duration = Duration::from_secs(100);

#[derive(Debug, State)]
pub struct InGameState {
    query: TextArea<'static>,
    score: usize,
    time_end: Instant,
    is_schema_table_visible: bool,
    focused_element: usize,
    question: model::Question,
    question_index: usize,
    execution_option: usize,
    is_done: bool,
}

impl InGameState {
    pub async fn default() -> Result<Self> {
        Ok(Self {
            query: Default::default(),
            score: Default::default(),
            time_end: Instant::now() + TIME_LIMIT,
            is_schema_table_visible: Default::default(),
            focused_element: Default::default(),
            question: util::get_question(1).await?,
            question_index: 1,
            execution_option: Default::default(),
            is_done: Default::default(),
        })
    }

    pub fn get_time_left(&self) -> Duration {
        self.time_end.saturating_duration_since(Instant::now())
    }
}

pub struct InGame<'a> {
    pub in_game_state: Res<'a, InGameState>,
    pub global_state: Res<'a, GlobalState>,
}

impl Widget for InGame<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(7),
                    Constraint::Percentage(68),
                    Constraint::Percentage(25),
                ]
                .as_ref(),
            )
            .split(area);
        let status_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(5),
                    Constraint::Percentage(10),
                    Constraint::Percentage(85),
                ]
                .as_ref(),
            )
            .split(main_area[0]);
        let query_and_question_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_area[1]);
        let result_and_features_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_area[2]);

        let in_game_state = &self.in_game_state;

        Score { in_game_state }.render(status_area[0], buf);

        Timer { in_game_state }.render(status_area[1], buf);

        HotKeyGuild {}.render(status_area[2], buf);

        QueryInput { in_game_state }.render(query_and_question_area[0], buf);

        Question { in_game_state }.render(query_and_question_area[1], buf);
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
