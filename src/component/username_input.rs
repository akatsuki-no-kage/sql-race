use ratatui::style::{Color, Style};
use tui_realm_stdlib::Input;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, Borders, InputType, TextModifiers},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct UsernameInput {
    component: Input,
}

impl Default for UsernameInput {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .title("Name", Alignment::Center)
                .input_type(InputType::Text)
                .placeholder(
                    "Input your name",
                    Style::new()
                        .fg(Color::DarkGray)
                        .add_modifier(TextModifiers::DIM),
                )
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl Component<Message, NoUserEvent> for UsernameInput {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        let cmd_result = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                ..
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Type(ch)),
            _ => CmdResult::None,
        };

        match cmd_result {
            CmdResult::Submit(State::One(StateValue::String(name))) => Some(Message::Play(name)),
            _ => Some(Message::None),
        }
    }
}
