pub mod component;

use std::time::{Duration, Instant};

use anyhow::Result;
use component::{
    action::Action, hotkey_guide::HotKeyGuild, query_input::QueryInput, question::Question,
    schema::Schema, score::Score, table::Table, timer::Timer,
};
use futures::{stream::FuturesOrdered, TryStreamExt};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    widgets::{ScrollbarState, TableState, Widget},
};
use sqlx::sqlite::SqliteRow;
use tui_textarea::TextArea;
use widgetui::{Events, Res, ResMut, State, WidgetResult};

use crate::{
    model,
    state::{GlobalState, Screen},
    util,
};

const TIME_LIMIT: Duration = Duration::from_secs(100);

#[derive(State)]
pub struct InGameState {
    query: TextArea<'static>,
    score: usize,
    time_end: Instant,
    is_schema_table_visible: bool,
    focused_element: usize,
    questions: Vec<model::Question>,
    question_index: usize,
    run_option: usize,
    is_popup_visible: bool,
    table_headers: Vec<String>,
    table_rows: Result<Vec<SqliteRow>>,
    table_state: TableState,
    table_scroll_state: ScrollbarState,
    schema_index: usize,
}

const QUESTION_COUNT: usize = 10;
const COMPONENT_COUNT: usize = 4;

impl InGameState {
    pub async fn default() -> Result<Self> {
        let questions: Vec<_> = (1..=10)
            .map(|i| util::get_question(i))
            .collect::<FuturesOrdered<_>>()
            .try_collect()
            .await?;

        Ok(Self {
            query: Default::default(),
            score: Default::default(),
            time_end: Instant::now() + TIME_LIMIT,
            is_schema_table_visible: Default::default(),
            focused_element: Default::default(),
            questions,
            question_index: Default::default(),
            run_option: Default::default(),
            is_popup_visible: Default::default(),
            table_headers: Default::default(),
            table_rows: Ok(vec![]),
            table_state: Default::default(),
            table_scroll_state: Default::default(),
            schema_index: Default::default(),
        })
    }

    pub fn reset(&mut self) {
        self.query = Default::default();
        self.score = Default::default();
        self.time_end = Instant::now() + TIME_LIMIT;
        self.is_schema_table_visible = Default::default();
        self.focused_element = Default::default();
        self.question_index = Default::default();
        self.run_option = Default::default();
        self.is_popup_visible = Default::default();
        self.table_headers = Default::default();
        self.table_rows = Ok(vec![]);
        self.table_state = Default::default();
        self.table_scroll_state = Default::default();
        self.schema_index = Default::default();
    }

    pub fn get_time_left(&self) -> Duration {
        self.time_end.saturating_duration_since(Instant::now())
    }

    pub fn focus_next(&mut self) {
        self.focused_element = (self.focused_element + 1) % COMPONENT_COUNT;
    }

    pub fn focus_previous(&mut self) {
        self.focused_element = (self.focused_element + COMPONENT_COUNT - 1) % COMPONENT_COUNT;
    }

    pub fn run_query(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn view_schema(&mut self) {
        self.is_popup_visible = true
    }

    pub fn submit(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn next_schema(&mut self) {
        let schema_count = self.questions[self.question_index].schemas.len();
        self.schema_index = (self.schema_index + 1) % schema_count;
    }

    pub fn previous_schema(&mut self) {
        let schema_count = self.questions[self.question_index].schemas.len();
        self.schema_index = (self.schema_index + schema_count - 1) % schema_count;
    }
}

pub struct InGame<'a> {
    pub in_game_state: ResMut<'a, InGameState>,
    pub global_state: Res<'a, GlobalState>,
}

impl Widget for InGame<'_> {
    fn render(mut self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        if self.in_game_state.is_popup_visible {
            let in_game_state = &self.in_game_state;
            Schema { in_game_state }.render(area, buf);
            return;
        }

        let main_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
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

        let in_game_state = &mut self.in_game_state;

        Score { in_game_state }.render(status_area[0], buf);

        Timer { in_game_state }.render(status_area[1], buf);

        HotKeyGuild {}.render(status_area[2], buf);

        QueryInput { in_game_state }.render(query_and_question_area[0], buf);

        Question { in_game_state }.render(query_and_question_area[1], buf);

        Action { in_game_state }.render(result_and_features_area[1], buf);

        Table { in_game_state }.render(result_and_features_area[0], buf);
    }
}

pub fn event_handler(
    events: Res<Events>,
    mut in_game_state: ResMut<InGameState>,
    mut global_state: ResMut<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame || in_game_state.is_popup_visible {
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
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.focus_previous(),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.focus_next(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.view_schema(),
        _ => {}
    }

    Ok(())
}
