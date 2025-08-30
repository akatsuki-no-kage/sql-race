use tui_realm_stdlib::Paragraph;
use tuirealm::{
    Component, Event, MockComponent, NoUserEvent,
    props::{Alignment, BorderSides, Borders, TextSpan},
};

use crate::app::Message;

#[derive(MockComponent)]
pub struct Question {
    component: Paragraph,
}

impl Question {
    pub fn new(question: String) -> Self {
        Self {
            component: Paragraph::default()
                .borders(Borders::default().sides(BorderSides::all()))
                .title("Question", Alignment::Center)
                .text(question.lines().map(TextSpan::new))
                .wrap(false),
        }
    }
}

impl Component<Message, NoUserEvent> for Question {
    fn on(&mut self, _: Event<NoUserEvent>) -> Option<Message> {
        Some(Message::None)
    }
}
