pub mod component;

use std::time::{Duration, Instant};

use anyhow::{anyhow, Result};
use component::{
    action::{self, Action},
    hotkey_guide,
    query_input::{self, QueryInput},
    question::{self, Question},
    schema::Schema,
    score,
    table::{self, Table},
    timer,
};
use futures::{stream::FuturesOrdered, TryStreamExt};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
    widgets::{ScrollbarState, TableState, Widget},
};
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row};
use tui_textarea::TextArea;
use widgetui::{
    constraint, layout, set, App, Chunks, Events, Res, ResMut, Set, State, WidgetFrame,
    WidgetResult,
};

use crate::{
    model,
    state::{GlobalState, Screen},
    util,
};

use super::home::component::username_input;

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

    fn get_query(&self) -> String {
        self.query.lines().join("\n")
    }

    pub fn run_query(&mut self) {
        let query = self.get_query();
        let raw_schema = self.questions[self.question_index].raw_schema.clone();
        match util::run_async(async move { util::run_query(&query, &raw_schema).await }) {
            Ok(rows) => {
                // HACK: use another way to get headers
                self.table_headers = if let Some(row) = rows.first() {
                    row.columns()
                        .iter()
                        .map(|col| col.name().to_string())
                        .collect()
                } else {
                    vec![]
                };
                self.table_rows = Ok(rows);
            }
            Err(err) => self.table_rows = Err(err),
        };
    }

    pub fn view_schema(&mut self) {
        self.is_popup_visible = true
    }

    pub fn next_question(&mut self) {
        let question_count = self.questions.len();
        self.question_index = (self.question_index + 1).min(question_count - 1);

        self.query = Default::default();
        self.score = (self.score + 1).min(question_count);
        self.is_schema_table_visible = Default::default();
        self.run_option = Default::default();
        self.is_popup_visible = Default::default();
        self.table_headers = Default::default();
        self.table_rows = Ok(vec![]);
        self.table_state = Default::default();
        self.table_scroll_state = Default::default();
        self.schema_index = Default::default();
    }

    pub fn submit(&mut self) {
        let query = self.get_query();
        let question = &self.questions[self.question_index];
        let answer_query = question.answer.clone();
        let raw_schema = question.raw_schema.clone();

        match util::run_async(
            async move { util::is_correct(&query, &answer_query, &raw_schema).await },
        ) {
            Ok(true) => self.next_question(),
            Ok(false) => self.table_rows = Err(anyhow!("Wrong answer")),
            Err(error) => {
                self.table_rows = Err(error);
            }
        }
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
        let query_and_question_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_area[1]);
        let result_and_features_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(main_area[2]);

        let in_game_state = &mut self.in_game_state;

        QueryInput { in_game_state }.render(query_and_question_area[0], buf);

        Question { in_game_state }.render(query_and_question_area[1], buf);

        Action { in_game_state }.render(result_and_features_area[1], buf);
    }
}

pub fn chunk_generator(frame: Res<WidgetFrame>, mut chunks: ResMut<Chunks>) -> WidgetResult {
    let new_chunks = layout! {
        frame.size(),
        (#3) => { %5, %10, %85 },
        (%70) => { %70, %30 },
        (%25) => { %70, %30 }
    };

    chunks.register_chunk::<score::Chunk>(new_chunks[0][0]);
    chunks.register_chunk::<timer::Chunk>(new_chunks[0][1]);
    chunks.register_chunk::<hotkey_guide::Chunk>(new_chunks[0][2]);

    chunks.register_chunk::<query_input::Chunk>(new_chunks[1][0]);
    chunks.register_chunk::<question::Chunk>(new_chunks[1][1]);

    chunks.register_chunk::<table::Chunk>(new_chunks[2][0]);
    chunks.register_chunk::<action::Chunk>(new_chunks[2][1]);

    Ok(())
}

pub fn state_updater(
    in_game_state: ResMut<InGameState>,
    username_input_state: Res<username_input::CustomState>,
    mut global_state: ResMut<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }
    if in_game_state.question_index == in_game_state.questions.len() - 1
        || in_game_state.get_time_left() == Duration::ZERO
    {
        let username = username_input_state.get_username();
        let score = in_game_state.score;
        let pool = global_state.pool.clone();
        util::run_async(async move { model::Score::insert(username, score as i64, &pool).await })?;
        global_state.screen = Screen::Home;
    }

    Ok(())
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
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.run_query(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.view_schema(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => in_game_state.submit(),
        _ => {}
    }

    Ok(())
}

#[set]
pub fn InGameSet(app: App) -> App {
    app.states((
        timer::CustomState::default(),
        score::CustomState::default(),
        table::CustomState::default(),
    ))
    .widgets((
        chunk_generator,
        event_handler,
        state_updater,
        hotkey_guide::render,
        timer::render,
        score::render,
        table::render,
        table::event_handler,
        component::query_input::event_handler,
        component::action::event_handler,
        component::schema::event_handler,
        component::table::event_handler,
    ))
}
