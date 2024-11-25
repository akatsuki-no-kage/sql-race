use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};
use widgetui::{Chunks, Events, Res, ResMut, State, WidgetFrame, WidgetResult};

use crate::{
    page::in_game::FocusState,
    state::{GlobalState, Screen},
};

pub struct Chunk;

pub enum ActionType {
    Run,
    ViewSchema,
    Submit,
    Exit,
}

impl ToString for ActionType {
    fn to_string(&self) -> String {
        match self {
            ActionType::Run => "Run (Ctrl + R)",
            ActionType::ViewSchema => "View Schema (Ctrl + H)",
            ActionType::Submit => "Submit (Ctrl + S)",
            ActionType::Exit => "Exit (Ctrl + Q)",
        }
        .to_string()
    }
}

const ACTIONS: [ActionType; 4] = [
    ActionType::Run,
    ActionType::ViewSchema,
    ActionType::Submit,
    ActionType::Exit,
];

const ID: usize = 3;

#[derive(Default, State)]
pub struct CustomState {
    selected_option: usize,
}

impl CustomState {
    fn next(&mut self) {
        self.selected_option = (self.selected_option + 1) % ACTIONS.len();
    }

    fn prev(&mut self) {
        let length = ACTIONS.len();
        self.selected_option = (self.selected_option + length - 1) % length;
    }
}

pub fn render(
    mut frame: ResMut<WidgetFrame>,
    chunks: Res<Chunks>,
    state: Res<CustomState>,
    focus_state: Res<FocusState>,
    global_state: Res<GlobalState>,
) -> WidgetResult {
    if global_state.screen != Screen::InGame {
        return Ok(());
    }

    let chunk = chunks.get_chunk::<Chunk>()?;

    let items: Vec<_> = ACTIONS
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let style = if i == state.selected_option {
                Style::default().bg(Color::Green)
            } else {
                Style::default()
            };
            ListItem::new(option.to_string()).style(style)
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .title("Options")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if focus_state.focused_element != ID {
                Color::White
            } else {
                Color::Green
            })),
    );
    frame.render_widget(list, chunk);

    Ok(())
}

pub fn event_handler(
    events: ResMut<Events>,
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

    match event {
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            ..
        }) => state.next(),
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            ..
        }) => state.prev(),
        // Event::Key(KeyEvent {
        //     code: KeyCode::Enter,
        //     modifiers: KeyModifiers::NONE,
        //     ..
        // }) => match ACTIONS[in_game_state.run_option] {
        //     ActionType::Run => in_game_state.run_query(),
        //     ActionType::ViewSchema => in_game_state.view_schema(),
        //     ActionType::Submit => in_game_state.submit(),
        //     ActionType::Exit => events.register_exit(),
        // },
        _ => {}
    }

    Ok(())
}
