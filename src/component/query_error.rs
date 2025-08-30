use tui_realm_stdlib::Textarea;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, TextSpan},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct QueryError {
    component: Textarea,
}

impl QueryError {
    pub fn new(error: rusqlite::Error) -> Self {
        Self {
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .sides(BorderSides::all())
                        .color(Color::Red),
                )
                .title("Question", Alignment::Center)
                .text_rows(error.to_string().lines().map(TextSpan::new)),
        }
    }
}

impl Component<Message, NoUserEvent> for QueryError {
    fn on(&mut self, event: Event<NoUserEvent>) -> Option<Message> {
        let _ = match event {
            Event::Keyboard(KeyEvent {
                code: Key::Down, ..
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Scroll(Direction::Up))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            _ => CmdResult::None,
        };
        Some(Message::None)
    }
}
