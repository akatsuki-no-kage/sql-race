pub mod component;

use component::{action, hotkey_guide, query_input, question, score, table, timer};
use ratatui::layout::{Constraint, Direction, Layout};
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

    return Ok(());
}

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

pub fn event_handler(
    events: Res<Events>,
    mut in_game_state: ResMut<FocusState>,
    mut global_state: ResMut<GlobalState>,
) -> WidgetResult {
    // if global_state.screen != Screen::InGame || in_game_state.is_popup_visible {
    //     return Ok(());
    // }
    //
    // let Some(event) = &events.event else {
    //     return Ok(());
    // };
    //
    // match event {
    //     Event::Key(KeyEvent {
    //         code: KeyCode::Char('q'),
    //         modifiers: KeyModifiers::CONTROL,
    //         ..
    //     }) => global_state.screen = Screen::Home,
    //     Event::Key(KeyEvent {
    //         code: KeyCode::Left,
    //         modifiers: KeyModifiers::CONTROL,
    //         ..
    //     }) => in_game_state.focus_previous(),
    //     Event::Key(KeyEvent {
    //         code: KeyCode::Right,
    //         modifiers: KeyModifiers::CONTROL,
    //         ..
    //     }) => in_game_state.focus_next(),
    //     Event::Key(KeyEvent {
    //         code: KeyCode::Char('r'),
    //         modifiers: KeyModifiers::CONTROL,
    //         ..
    //     }) => in_game_state.run_query(),
    //     Event::Key(KeyEvent {
    //         code: KeyCode::Char('h'),
    //         modifiers: KeyModifiers::CONTROL,
    //         ..
    //     }) => in_game_state.view_schema(),
    //     Event::Key(KeyEvent {
    //         code: KeyCode::Char('s'),
    //         modifiers: KeyModifiers::CONTROL,
    //         ..
    //     }) => in_game_state.submit(),
    //     _ => {}
    // }

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
        action::CustomState::default(),
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
        action::render,
        action::event_handler,
    ))
}
