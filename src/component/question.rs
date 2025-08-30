use tui_realm_stdlib::Textarea;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
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
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
