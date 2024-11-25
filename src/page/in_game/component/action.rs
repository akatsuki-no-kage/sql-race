use ratatui::{
    buffer::Buffer,
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Widget},
};
use widgetui::{Events, Res, ResMut, WidgetResult};

use crate::{
    page::in_game::InGameState,
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

pub struct Action<'a> {
    pub in_game_state: &'a InGameState,
}

impl Widget for Action<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let items: Vec<_> = ACTIONS
            .iter()
            .enumerate()
            .map(|(i, option)| {
                let style = if i == self.in_game_state.run_option {
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
                .border_style(
                    Style::default().fg(if self.in_game_state.focused_element != ID {
                        Color::White
                    } else {
                        Color::Green
                    }),
                ),
        );
        list.render(area, buf);
    }
}

impl InGameState {
    fn next_run_option(&mut self) {
        self.run_option = (self.run_option + 1) % ACTIONS.len();
    }

    fn previous_run_option(&mut self) {
        let length = ACTIONS.len();
        self.run_option = (self.run_option + length - 1) % length;
    }
}

pub fn event_handler(
    mut events: ResMut<Events>,
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
        Event::Key(KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            in_game_state.next_run_option();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            ..
        }) => {
            in_game_state.previous_run_option();
        }
        Event::Key(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            ..
        }) => match ACTIONS[in_game_state.run_option] {
            ActionType::Run => in_game_state.run_query(),
            ActionType::ViewSchema => in_game_state.view_schema(),
            ActionType::Submit => in_game_state.submit(),
            ActionType::Exit => events.register_exit(),
        },
        _ => {}
    }

    Ok(())
}
