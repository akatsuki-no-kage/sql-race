use tui_realm_stdlib::Textarea;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    command::{Cmd, CmdResult, Direction, Position},
    event::{Key, KeyEvent},
    props::{Alignment, BorderSides, Borders, Color, Style, TextSpan},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct Question {
    component: Textarea,
}

impl Question {
    pub fn new(question: String) -> Self {
        Self {
            component: Textarea::default()
                .borders(
                    Borders::default()
                        .sides(BorderSides::all())
                        .color(Color::Green),
                )
                .inactive(Style::reset())
                .title("Question", Alignment::Center)
                .text_rows(question.lines().map(TextSpan::new)),
        }
    }
}

impl Component<Message, NoUserEvent> for Question {
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
