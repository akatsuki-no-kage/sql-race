pub mod component;

use anyhow::anyhow;
use component::{hotkey_guide, query_input, question, score, table, timer};
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Direction, Layout},
};
use sqlx::{Column, Row};
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

const COMPONENT_COUNT: usize = 4;

#[derive(Default, State)]
pub struct FocusState {
    focused_element: usize,
}

impl FocusState {
    fn next(&mut self) {
        self.focused_element = (self.focused_element + 1) % COMPONENT_COUNT;
    }

    fn prev(&mut self) {
        self.focused_element = (self.focused_element + COMPONENT_COUNT - 1) % COMPONENT_COUNT;
    }
}

pub fn chunk_generator(
    frame: Res<WidgetFrame>,
    mut chunks: ResMut<Chunks>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let new_chunks = layout! {
        frame.size(),
        (#3) => { %5, %10, %85 },
        (>0) => { %70, %30 }
    };

    let sub_chunks = layout! {
        new_chunks[1][0],
        (%70),
        (%25)
    };

    chunks.register_chunk::<score::Chunk>(new_chunks[0][0]);
    chunks.register_chunk::<timer::Chunk>(new_chunks[0][1]);
    chunks.register_chunk::<hotkey_guide::Chunk>(new_chunks[0][2]);

    chunks.register_chunk::<question::Chunk>(new_chunks[1][1]);
    chunks.register_chunk::<query_input::Chunk>(sub_chunks[0][0]);
    chunks.register_chunk::<table::Chunk>(sub_chunks[1][0]);

    Ok(())
}

pub fn run_query(
    query_state: &query_input::CustomState,
    question_state: &question::CustomState,
    table_state: &mut table::CustomState,
) {
    let query = query_state.to_string();
    let raw_schema = question_state.questions[question_state.selected_question]
        .raw_schema
        .clone();
    match util::run_async(async move { util::run_query(&query, &raw_schema).await }) {
        Ok(rows) => {
            // HACK: use another way to get headers
            table_state.headers = if let Some(row) = rows.first() {
                row.columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect()
            } else {
                vec![]
            };
            table_state.rows = Ok(rows);
        }
        Err(err) => table_state.rows = Err(err),
    };
}

fn submit(
    username_input_state: &username_input::CustomState,
    query_state: &query_input::CustomState,
    question_state: &mut question::CustomState,
    table_state: &mut table::CustomState,
    score_state: &mut score::CustomState,
    global_state: &mut GlobalState,
) -> WidgetResult {
    if question_state.selected_question > question::QUESTION_COUNT {
        return Ok(());
    }

    let query = query_state.to_string();
    let question = &question_state.questions[question_state.selected_question];
    let answer_query = question.answer.clone();
    let raw_schema = question.raw_schema.clone();

    match util::run_async(async move { util::is_correct(&query, &answer_query, &raw_schema).await })
    {
        Ok(true) => {
            if question_state.selected_question == question::QUESTION_COUNT {
                return finish_game(username_input_state, score_state, global_state);
            }
            score_state.score += 1;
            question_state.selected_question += 1;
        }
        Ok(false) => table_state.rows = Err(anyhow!("Wrong answer")),
        Err(error) => {
            table_state.rows = Err(error);
        }
    }

    Ok(())
}

fn finish_game(
    username_input_state: &username_input::CustomState,
    score_state: &score::CustomState,
    global_state: &mut GlobalState,
) -> WidgetResult {
    let username = username_input_state.get_username();
    let score = score_state.score;
    let pool = global_state.pool.clone();
    util::run_async(async move { model::Score::insert(username, score as i64, &pool).await })?;
    global_state.screen = Screen::Home;

    Ok(())
}

pub fn event_handler(
    events: Res<Events>,
    mut focus_state: ResMut<FocusState>,
    username_input_state: Res<username_input::CustomState>,
    mut score_state: ResMut<score::CustomState>,
    query_state: Res<query_input::CustomState>,
    mut question_state: ResMut<question::CustomState>,
    mut table_state: ResMut<table::CustomState>,
    mut global_state: ResMut<GlobalState>,
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
        }) => finish_game(&username_input_state, &score_state, &mut global_state)?,
        Event::Key(KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => focus_state.prev(),
        Event::Key(KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => focus_state.next(),
        Event::Key(KeyEvent {
            code: KeyCode::Char('r'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => run_query(&query_state, &question_state, &mut table_state),
        Event::Key(KeyEvent {
            code: KeyCode::Char('h'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => global_state.screen = Screen::Schema,
        Event::Key(KeyEvent {
            code: KeyCode::Char('s'),
            modifiers: KeyModifiers::CONTROL,
            ..
        }) => submit(
            &username_input_state,
            &query_state,
            &mut question_state,
            &mut table_state,
            &mut score_state,
            &mut global_state,
        )?,
        _ => {}
    }

    Ok(())
}

#[set]
pub fn InGameSet(app: App) -> App {
    app.states((
        FocusState::default(),
        timer::CustomState::default(),
        score::CustomState::default(),
        query_input::CustomState::default(),
        question::CustomState::default(),
        table::CustomState::default(),
    ))
    .widgets((
        chunk_generator,
        event_handler,
        timer::render,
        timer::state_updater,
        score::render,
        hotkey_guide::render,
        query_input::render,
        query_input::event_handler,
        question::render,
        table::render,
        table::event_handler,
    ))
}
