use ratatui::style::{Color, Style};
use tui_realm_stdlib::Input;
use tuirealm::{
    Component, Event, MockComponent,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderSides, Borders, InputType, TextModifiers},
};

use crate::{Message, event::UserEvent};

#[derive(MockComponent)]
pub struct NameInput {
    component: Input,
}

impl Default for NameInput {
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

impl Component<Message, UserEvent> for NameInput {
    fn on(&mut self, event: Event<UserEvent>) -> Option<Message> {
        let _ = match event {
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
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Type(ch)),
            _ => CmdResult::None,
        };
        Some(Message::None)
    }
}
